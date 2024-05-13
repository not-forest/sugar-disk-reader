//! Main application interfaces for connection with a target device.
//!
//! This module handles all bridge related tasks, that includes connecting, disconnecting and
//! handling all commands that are coming from the target device and from the mobile device. 

use std::sync::Arc;
use std::any::Any;
use std::thread;

use tokio::sync::{Mutex, mpsc::{self, Sender}};
use hidapi::{BusType, HidApi, HidDevice, HidError};

use crate::sugar::conn::buf::BufferError;
use crate::sugar::parse::SugarParser;
use super::{
    buf::{Buffer, USBV2Buf},
    cmd::DaemonCommand,
};

pub type BridgeResult<T> = Result<T, BridgeError>;
type DataBuffer = Arc<Mutex<Box<dyn Buffer>>>;
type Device = Arc<Mutex<HidDevice>>;
type Tx = Option<Sender<DaemonCommand>>;

const CHANNEL_BUFFER_SIZE: usize = 1024;

/// Custom error type for bridge communication.
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
}

/// A custom structure that is being created on each communication between target devices.
///
/// Each new connection a new bridge is being transformed, while the daemon also expects only one
/// connection bridge with an exact ID, already known for it. All other bridges with a wrong id
/// won't be able to connect.
pub struct Bridge {
    _id: usize,
    tx: Tx,

    buf: DataBuffer,
    device: Device,
}

impl Bridge {
    /// Initializes the bridge and finds a proper device, which is connected to the USB port.
    ///
    /// This method will return a new bridge, which is not yet connected, but ready to do so. If
    /// not USB devices are available the result will be Ok(None).
    ///
    /// # Warn
    ///
    /// This method does not connect to the target right away, but only obtains all required
    /// information for a proper communication.
    pub fn new(id: usize) -> Result<Option<Self>, HidError> {
        log::info!("Creating a new communication bridge");
        let api = HidApi::new_without_enumerate()?;
        // Only one device is logical in our case, because the host is a mobile device.
        if let Some(dev_info) = api.device_list()
                .into_iter()
                .map(|d| { log::info!("dev: {:#?}", d); d })
                .find(|d| d.bus_type().type_id() == BusType::Usb.type_id()) // Finding the first USB connected device.
        {
            let dev = dev_info.open_device(&api)?;  // Initializing the HIDAPI.

            #[cfg(debug_assertions)]
            if let Ok(dev_i) = dev.get_device_info() {
                let vid = dev_i.vendor_id();
                let pid = dev_i.product_id();
                let int = dev_i.interface_number();

                let dev_m = dev.get_manufacturer_string()
                    .unwrap_or_else(|e| Some(e.to_string())).unwrap();
                let dev_s = dev.get_serial_number_string()
                    .unwrap_or_else(|e| Some(e.to_string())).unwrap();

                log::info!("Obtained a possible candidate for connection:\n
                    VID: {}; PID: {};\n
                    Interface number: {}\n
                    Manufacturer: {}\n
                    Serial number: {}.",
                    vid, pid, int, dev_m, dev_s);
            }


            return Ok(Some(Self::new_v2(dev, id)))
        }

        log::info!("No devices connected to the port.");
        Ok(None)
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

        // Checks for the upcoming stream from the target device.
        for i in 0..cpus.into() {
            log::info!("Spawning listener thread: {}", i);
            let buf_lock = self.buf.clone();
            let dev_lock = self.device.clone();
            let tx = tx.clone();

            // Spawning 
            tokio::spawn(async move {
                loop {
                    // Obtaining the locks.
                    let mut buffer = buf_lock.lock().await;
                    let dev = dev_lock.lock().await;

                    // If new data exist, reading it.
                    match buffer.read(&dev) {
                        Ok(bytes) => {
                            log::info!("Thread ({}): Obtained oncoming data: {}", i, bytes.escape_ascii());
                            // Sending the command for parsing.
                            if let Err(tx_err) = tx.send(DaemonCommand::new(bytes)).await {
                                log::error!("Thread ({}): Unable to send data: {}, channel is closed, aborting...", i, tx_err);
                                break; // Breaking out of the loop awaits the data.
                            }
                        },
                        Err(buf_err) => {
                            match buf_err {
                                BufferError::NoData => continue,
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

        // All obtained bytes are then parsed and sent to the front-end or to the target device.
        while let Some(bytes) = rx.recv().await {
            SugarParser::parse_byte_code(self, bytes);
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

    /// Creates a new bridge connection via usb v2.0 cable. 
    fn new_v2(dev: HidDevice, id: usize) -> Self {
        Self {
            _id: id,
            tx: None,
            buf: Arc::new(Mutex::new(Box::new(USBV2Buf::default()))),
            device: Arc::new(Mutex::new(dev)),
        }
    }
}

pub mod service {
    use hidapi::HidError;

    use super::{Bridge, BridgeError};

    #[tokio::main]
    pub async fn connect() -> ConnectionStatus {
        let id = rand::random();

        match Bridge::new(id) {
            Ok(op) => match op {
                Some(mut bridge) => {
                    if let Err(err) = bridge.connect().await {
                        match err {
                            BridgeError::BridgeClosed => ConnectionStatus::BridgeClosed,
                            BridgeError::ConnectionRefused => ConnectionStatus::Refused,
                            BridgeError::ConnectionTimeout => ConnectionStatus::Timeout,
                            _ => unreachable!()
                        }
                    } else {
                        ConnectionStatus::Connected
                    }
                },
                None => ConnectionStatus::NoDevice,
            },
            Err(_err) => todo!(),
        }
    }

    pub async fn disconnect() -> ConnectionStatus {
       unimplemented!() 
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
    }
}
