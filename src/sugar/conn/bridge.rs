//! Main application interfaces for connection with a target device.
//!
//! This module handles all bridge related tasks, that includes connecting, disconnecting and
//! handling all commands that are comming from the target device and from the mobilw device. 

use std::sync::Arc;
use std::any::Any;
use std::thread;
use std::mem;

use tokio::sync::{Mutex, mpsc::{self, Sender}};
use hidapi::{BusType, HidApi, HidDevice, HidError};

use crate::sugar::conn::buf::BufferError;
use super::buf::{Buffer, USBV2Buf};

pub type BridgeResult<T> = Result<T, BridgeError>;
type DataBuffer = Arc<Mutex<Box<dyn Buffer>>>;
type Device = Arc<Mutex<HidDevice>>;
type Tx = Option<Sender<DaemonCommand>>;

const CHANNEL_BUFFER_SIZE: usize = 1024;

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
        let api = HidApi::new()?;
        // Only one device is logical in our case, because the host is a mobile device.
        if let Some(dev_info) = api.device_list()
                .into_iter()
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

        Ok(None) // No devices connected to the port.
    } 
    
    /// Connects the existing bridge to start the communication.
    ///
    /// While connected, listens to any upcoming data from the target machine as well as from the
    /// host device. 
    pub async fn connect(&mut self) -> BridgeResult<()> {
        let cpus = thread::available_parallelism().unwrap();
        let (tx, mut rx) = mpsc::channel::<DaemonCommand>(CHANNEL_BUFFER_SIZE);

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
                            // Sending the command for parsing.
                            tx.send(DaemonCommand::new(bytes));
                        },
                        Err(hid_err) => match hid_err {
                            _ => (),
                        },
                    };
                }
            });
        }
        self.tx.replace(tx); // after this replacement, it is possible to disconnect.

        // All obtained bytes are then parsed and sent to the front-end or to the target device.
        while let Some(bytes) = rx.recv().await {
            unimplemented!()
        }

        Ok(()) // A properly closed bridge.
    }

    /// Disconnects the communication by sending a shutdown command.
    ///
    /// If the Tx is not ready yet, halts until it appears.
    pub async fn disconnect(&self) -> BridgeResult<()> {
        if let Some(tx) = &self.tx {
            tx.send(DaemonCommand::user_disconnect());

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

/// Custom error type for bridge communication.
pub enum BridgeError {
    /// Appears when trying to close a bridge, which is yet not initialized fully.
    BridgeNotReady,
    /// Appears when the target device has refused a connection. Can happen when trying to connect
    /// to a device when it is already is connected or when the bridge ID does not match.
    ConnectionRefused,
    /// Connection to the device has timed out.
    ConnectionTimeout,
}

/// A bytecode command that is being used to communicate between two devices.
///
/// All commands are represented as a set of bytes, where size decides how much bytes are in the
/// command including the size byte and a checksum.
///
/// # Representation:
///
///          (opt)                     (opt)
/// *------*--------*---------*------*----------*
/// | SIZE | PREFIX | COMMAND | DATA | CHECKSUM |
/// *------*--------*---------*------*----------*
///
/// The size of the command can vary a lot based on the command itself and data that comes with it.
/// Some fields like a prefix and data can be optional. Commands are parsed sequentially, therefore
/// they can be stacked.
#[repr(transparent)]
pub struct DaemonCommand(Vec<DaemonCommandByte>);

impl DaemonCommand {
    /// Creates a new daemon command based on the obtained slice of bytes. 
    pub fn new(slice: &[u8]) -> Self {
        Self(unsafe{ mem::transmute(slice.to_vec()) })
    }

    pub fn init(id: usize) -> Self {
        use DaemonCommandByte::*;
        let id = id.to_ne_bytes();

        unimplemented!()
        //crate::dcommand!(REQ, CONN, BID, id)
    }

    /// This command is being sent only by user from the front-end side.
    pub fn user_disconnect() -> Self {
        use DaemonCommandByte::*;
        // Triple acknowledgement is a high priority operation that will always mean that the user
        // wants to perform something. In this case the shutdown.
        crate::dcommand!(SHUT, ACK, ACK, ACK)
    }
}

/// Convenient macro to create a custom command
///
/// It allows for any amount of arguments, as long as it is less than u8::MAX;
#[macro_export]
macro_rules! dcommand {
    ($($args:tt),*) => {{
        let all = [$($args),*];
        let size = all.len() + 2;
    
        assert!(size < u8::MAX.into(), "The amount of bytes in one command cannot be bigger than u8::MAX.");

        let mut v = Vec::with_capacity(size);
        let checksum = size as u8;

        v.push(size.into()); // Pushing the size first.
        for arg in all {
            // Pushing all 
            checksum.wrapping_add(Into::<u8>::into(arg));
            v.push(arg)
        }

        // Obtaining the amount of bytes to add for this command, so that the overall sum will be 0
        let checksum = u8::MAX - checksum;
        v.push(checksum.into());

        DaemonCommand(v)
    }};
    () => (unreachable!());
}

/// A byte value of a command for both daemon and an application to communicate. 
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum DaemonCommandByte {
    // Prefixes (Second byte of the command.)

    /// Request to do something that requires an acknowledgement.
    REQ =   0x00,
    /// Acknowledgement from the other side that allows to proceed to execute.
    ACK =   0x01,
    /// No acknowledgement, means no execution will happen and may come with additional info
    NACK =  0x02,

    // Helpers

    /// The size of something that comes then after the next byte after the next one. Basically
    /// that means that the next byte is the amount of bytes to read and those bytes must be
    /// represented as something.
    SIZE =   0xff,

    // Commands
    
    /// Asks for a connection. Must be performed at the very start. Bridge's ID comes after this
    /// command.
    CONN =  0x03,
    /// Asks for a proper shutdown. Will close the connection and shutdown the daemon software.
    SHUT =  0x04,

    /// Select the disk or partition. After this command, daemon will expect the disk number or
    /// partition name.
    SEL =   0x05,
    /// Removes the selection of the disk or partition.
    UNSEL = 0x06,
    /// Reads files. The following data might vary.
    READ =  0x07,

    // Data parse prefix

    /// Name comes after this byte.
    NAME =  0x20,
    /// Partition
    PART =  0x21,
    /// File
    FILE =  0x22,
    /// Directory
    DIR =   0x23,
    /// Bridge's id.
    BID =   0x24,
}

impl Into<u8> for DaemonCommandByte {
    fn into(self) -> u8 {
        self as u8
    }
}

impl Into<DaemonCommandByte> for u8 {
    fn into(self) -> DaemonCommandByte {
        unsafe{ mem::transmute(self) }
    }
}


impl Into<DaemonCommandByte> for usize {
    fn into(self) -> DaemonCommandByte {
        unsafe{ mem::transmute(self as u8) }
    }
}
