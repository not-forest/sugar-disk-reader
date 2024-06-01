//! Custom module for parsing daemon-mobile communication byte code.

use super::conn::{cmd::{DaemonCommand, DaemonCommandByte}, Bridge, Buffer};

/// Struct which handles all parsing activity related to user input and data.
///
/// This struct is ZST, therefore it will be optimized by the compiler to only provide linking to
/// all it's related functions.
pub struct SugarParser;

impl SugarParser {
    /// Parses the daemon byte communication code and based on the result, calls different
    /// functions and changes the state of the communication bridge.
    pub async fn parse_byte_code(bridge: &mut Bridge, command: DaemonCommand) -> ParseOutput {
        use DaemonCommandByte::*;

        let res = DaemonCommand::blank();
        let mut cmd_iter = command.byte_code().to_owned().into_iter();
        let mut _size: u8 = 0;
        let mut _count: u8 = 0;
        let mut _prev_byte = None;

        loop {
            match cmd_iter.next() {
                Some(byte) => {
                    _count = _count.wrapping_add(byte.into());

                    match byte {
                        prefix @ (REQ | ACK | NACK) => {
                            _prev_byte.replace(prefix);
                        }
                        data => {
                            if _prev_byte.is_none() {
                                _size = data.into();
                                _prev_byte.replace(SIZE);
                            }
                            break;
                        }
                    }
                }
                None => {
                    let mut buffer = bridge.buf.lock().await;
                    let device = bridge.device.lock().await;

                    let (response, output) = if _count == 0 {
                        if _prev_byte.is_some() {
                            (res, ParseOutput::Success)
                        } else {
                            (DaemonCommand::retry(), ParseOutput::Empty)
                        }
                    } else {
                        (DaemonCommand::retry(), ParseOutput::Empty)
                    };

                    while let Err(err) = buffer.write(&device, response.clone()) {
                        log::error!("Unable to write data to the target device: {}", err);
                    }

                    return output;
                }
            }
        }

        ParseOutput::UnparsableTokens
    }
}

/// An output from the parser that is either an error or a success.
pub enum ParseOutput {
    /// The command is properly handled.
    Success,
    /// The checksum is not right. Some bytes were badly transfered.
    Checksum,
    /// The command is empty.
    Empty,
    /// Unable to parse the command. 
    UnparsableTokens,
}
