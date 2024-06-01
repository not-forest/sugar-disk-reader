//! Main application interfaces for connection with a target device.
//!
//! This module handles all bridge related tasks, that includes connecting, disconnecting and
//! handling all commands that are coming from the target device and from the mobile device. 

use std::sync::Arc;
use std::thread;
use tokio::sync::{Mutex, mpsc::{self, Sender}};
use rusb::{Context, DeviceDescriptor, DeviceHandle, UsbContext};
use lazy_static::lazy_static;

use crate::sugar::parse::SugarParser;
use super::{
    buf::{Buffer, USBV2Buf},
    cmd::DaemonCommand,
};

pub type BridgeResult<T> = Result<T, BridgeError>;
type DataBuffer = Arc<Mutex<Box<dyn Buffer>>>;
type Device = Arc<Mutex<DeviceHandle<rusb::Context>>>;
type Tx = Option<Sender<DaemonCommand>>;

const CHANNEL_BUFFER_SIZE: usize = 1024;

lazy_static! {
    /// Static reference to a bridge connection.
    static ref BRIDGE_REF: Arc<Mutex<Option<Bridge>>> = Arc::new(Mutex::new(None));
}

/// Custom error type for bridge communication.
#[derive(Debug)]
pub enum BridgeError {
    /// Appears when trying to close a bridge, which is yet not initialized fully.
    BridgeNotReady,
    /// Appears when the target device has refused a connection. Can happen when trying to connect
    /// to a device when it is already is connected or when the bridge ID does not match.
    ConnectionRefused,
    /// Connection to the device has timed out.
    ConnectionTimeout,
    /// Unable to send any data to the bridge since it is closed.
    BridgeClosed,
    /// Unable to setup a new libusb context
    ContextError,
    /// Unable to read a usb device by a file descriptor.
    FileDescriptorError
}

/// A custom structure that is being created on each communication between target devices.
///
/// Each new connection a new bridge is being transformed, while the daemon also expects only one
/// connection bridge with an exact ID, already known for it. All other bridges with a wrong id
/// won't be able to connect.
pub struct Bridge {
    _id: usize,
    tx: Tx,

    pub buf: DataBuffer,
    pub device: Device,
    pub dev_desc: DeviceDescriptor,
}

impl Bridge {
    /// Initializes the bridge and finds a proper device, which is connected to the USB port.
    ///
    /// This method will return a new bridge, which is not yet connected, but ready to do so. If
    /// not USB devices are available the result will be Ok(None). The file descriptor must be
    /// provided from the Java interface, because only Java android code has permissions to obtain
    /// the device.
    ///
    /// # Warn
    ///
    /// This method does not connect to the target right away, but only obtains all required
    /// information for a proper communication.
    pub fn new(id: usize, fd: i32) -> BridgeResult<Self> {
        log::info!("Creating a new communication bridge");
        #[cfg(debug_assertions)]
        rusb::disable_device_discovery().map_err(|err| {
            log::error!("Bridge error: Unable to disable device discovery: {}", err);
            BridgeError::ContextError
        })?;    // This is required since we already have a file desriptor.

        // Creating a new libusb context.
        let mut context = Context::new().map_err(|err| {
            log::error!("Bridge error: Unable to create a new context: {}", err);
            BridgeError::ContextError
        })?;    // Clear libusb context.

        context.set_log_level(rusb::LogLevel::Debug);
        // Trying to obtain a device handle from the provided file descriptor.
        let devh = unsafe { context.open_device_with_fd(fd).map_err(|err| { 
            log::error!("Bridge error: Unable to open a device with provided file descriptor: {}", err);
            BridgeError::FileDescriptorError 
        })}?;

        let devd = devh.device();
        let devdc = devd.device_descriptor().map_err(|err| { 
            log::error!("Bridge error: Unable to obtain the device descriptor: {}", err);
            BridgeError::FileDescriptorError
        })?;

        log::debug!("Found device: Bus: {:03}, Addr: {:03}, ID: {:04x}:{:04x}\n
            Max supported USB version: {},", 
            devd.bus_number(), devd.address(), devdc.vendor_id(), devdc.product_id(), devdc.usb_version());

        let handle_arc = Arc::new(Mutex::new(devh));

        return Ok(Self {
            _id: id,
            tx: None,
            buf: Arc::new(Mutex::new(Box::new(USBV2Buf::default()))),
            device: handle_arc,
            dev_desc: devdc,
        });
    }
    /// Connects the existing bridge to start the communication.
    ///
    /// While connected, listens to any upcoming data from the target machine as well as from the
    /// host device. 
    pub async fn connect(&mut self) -> BridgeResult<()> {
        log::info!("Connecting to the bridge...");
        let cpus = thread::available_parallelism().unwrap();
        let (tx, mut rx) = mpsc::channel::<DaemonCommand>(CHANNEL_BUFFER_SIZE);
        log::info!("Available threads: {}", cpus);

        for i in 0..cpus.into() {
            log::info!("Spawning listener thread: {}", i);
            let buf_lock = self.buf.clone();
            let device_lock = self.device.clone();
            let tx = tx.clone();

            tokio::spawn(async move {
                loop {
                    let mut buffer = buf_lock.lock().await;
                    let mut device = device_lock.lock().await;

                    match buffer.read(&mut *device) {
                        Ok(bytes) => {
                            log::info!("Thread ({}): Obtained oncoming data: {}", i, bytes.escape_ascii());
                            if let Err(tx_err) = tx.send(DaemonCommand::new(bytes)).await {
                                log::error!("Thread ({}): Unable to send data: {}, channel is closed, aborting...", i, tx_err);
                                break;
                            }
                        },
                        Err(buf_err) => {
                            match buf_err {
                                rusb::Error::InvalidParam => continue,
                                _ => log::error!("Thread ({}): Error while reading the data from the USB bus: {:#?}", i, buf_err),
                            }
                        },
                    };
                }
            });
        }

        log::info!("Connection established. Writing the initialization command..."); 
        // Sending the init command right away. It must always be Ok since we havent even closed
        // the channel yet.
        tx.send(DaemonCommand::init(self._id)).await.ok();
        self.tx.replace(tx); // after this replacement, it is possible to disconnect. 

        let mut cmds = 0;

        // All obtained bytes are then parsed and sent to the front-end or to the target device.
        while let Some(bytes) = rx.recv().await {
            use crate::sugar::parse::ParseOutput;
            match SugarParser::parse_byte_code(self, bytes).await {
                ParseOutput::Success => { log::info!("Successfully parsed request number: {}", cmds); cmds += 1; },
                ParseOutput::Empty => log::warn!("Obtained empty command. Ignoring..."),
                ParseOutput::Checksum => log::error!("Obtained message has a wrong checksum. Sending a retry request."),
                ParseOutput::UnparsableTokens => log::error!("Obtained unparsable command. Please check the connection."),
            }
        }

        log::info!("Bridge is closed.");
        Ok(()) // A properly closed bridge.
    }

    /// Disconnects the communication by sending a shutdown command.
    ///
    /// If the Tx is not ready yet, halts until it appears.
    pub async fn disconnect(&self) -> BridgeResult<()> {
        if let Some(tx) = &self.tx {
            if let Err(tx_err) = tx.send(DaemonCommand::user_disconnect()).await {
                log::error!("Unable to disconnect the bridge: {}", tx_err);
                return Err(BridgeError::BridgeClosed);
            }

            Ok(())
        } else { Err(BridgeError::BridgeNotReady) }
    }
}

pub mod service {
    use super::{Bridge, BridgeError, BRIDGE_REF};

    #[tokio::main]
    pub async fn connect(fd: i32) -> ConnectionStatus {
        let id = rand::random();

        match Bridge::new(id, fd) {
            Ok(bridge) => {
                BRIDGE_REF.lock().await.replace(bridge);
                if let Err(_err) = BRIDGE_REF.lock().await.as_mut().unwrap().connect().await {
                    match _err {
                        BridgeError::BridgeClosed => ConnectionStatus::BridgeClosed,
                        BridgeError::ConnectionRefused => ConnectionStatus::Refused,
                        BridgeError::ConnectionTimeout => ConnectionStatus::Timeout,
                        BridgeError::FileDescriptorError => ConnectionStatus::WrongData,
                        BridgeError::ContextError => ConnectionStatus::InnerError,
                        e @ _ => {
                            log::error!("Unhandled error has occur: {:#?}", e);
                            unreachable!()
                        },
                    }
                } else {
                    ConnectionStatus::Connected
                }
            },
            Err(_err) => match _err {
                BridgeError::FileDescriptorError => ConnectionStatus::WrongData,
                BridgeError::ContextError => ConnectionStatus::InnerError,
                e @ _ => {
                    log::error!("Unhandled error has occur: {:#?}", e);
                    unreachable!()
                },
            } 
        }
    }

    /// Disconnects from the currently existing bridge.
    #[tokio::main]
    pub async fn disconnect() -> ConnectionStatus {
        let mut bridge_ref = BRIDGE_REF.lock().await;

        if let Some(bridge) = bridge_ref.as_mut() {
            match bridge.disconnect().await {
                Ok(_) => {
                    *bridge_ref = None;
                    ConnectionStatus::Disconnected
                },
                Err(_err) => match _err {
                    BridgeError::BridgeClosed => ConnectionStatus::BridgeClosed,
                    BridgeError::ContextError => ConnectionStatus::InnerError,
                    e @ _ => {
                        log::error!("Unhandled error has occurred: {:#?}", e);
                        unreachable!()
                    },
                },
            }
        } else {
            ConnectionStatus::Disconnected
        }
    }

    /// Gets info about a current connection.
    #[tokio::main]
    pub async fn get_conn_info() -> String {
        let mut bridge_ref = BRIDGE_REF.lock().await;

        if let Some(bridge) = bridge_ref.as_mut() {
            let devd = bridge.device.lock().await.device();
            let devdc = &bridge.dev_desc;
            format!("Bus: {:03}, Addr: {:03}, ID: {:04x}:{:04x}\nMax supported USB version: {}.", 
                devd.bus_number(), devd.address(), devdc.vendor_id(), devdc.product_id(), devdc.usb_version());        
        }

        String::new()
    }

    /// Local enum to represent the current status of the connection.
    #[repr(u8)]
    pub enum ConnectionStatus {
        Connected           = 0, 
        Disconnected        = 1, 
        NoDevice            = 2, 
        BridgeClosed        = 3, 
        Refused             = 4,  
        Timeout             = 5, 
        WrongData           = 6,
        InnerError          = 7,
    }
}
