//! This module defines a bytecode communication language for communication between the daemon and
//! the mobile device. The command then has to be parsed on both sides to perform different tasks.

use std::mem;

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
    
    /// Pushes new data before the checksum while counting the new one.
    ///
    /// This allows to push new data into the command. This must not be used to stack commands.
    pub fn push_data(&mut self, data: &[u8]) {
        let mut checksum = u8::MAX - Into::<u8>::into(self.0.pop().unwrap());  // Commands would never be empty.
        for byte in data.to_owned() {
            let _ = checksum.wrapping_add(byte);
            self.0.push(byte.into());
        }
        checksum = u8::MAX - checksum;
        self.0.push(checksum.into());
    }

    /// Returns the slice of byte code written in the command.
    pub fn byte_code(&self) -> &[DaemonCommandByte] {
        self.0.as_ref()
    }

    /// Creates a initialization command that the daemon expects from the target's side.
    pub fn init(id: usize) -> Self {
        use DaemonCommandByte::*;
        let mut cmd = crate::dcommand!(REQ, CONN, BID);
        cmd.push_data(&id.to_ne_bytes());
        cmd
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
        let all = [$($args.into()),*];
        let size = all.len() + 2;
    
        assert!(size < u8::MAX.into(), "The amount of bytes in one command cannot be bigger than u8::MAX.");

        let mut v = Vec::with_capacity(size);
        let checksum = size as u8;

        v.push(size.into()); // Pushing the size first.
        for arg in all {
            // Pushing all 
            let _ = checksum.wrapping_add(Into::<u8>::into(arg));
            v.push(arg)
        }

        // Obtaining the amount of bytes to add for this command, so that the overall sum will be 0
        let checksum = u8::MAX - checksum;
        v.push(checksum.into());

        DaemonCommand(v)
    }};
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
