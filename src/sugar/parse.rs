//! Custom module for parsing daemon-mobile communication byte code.

use super::conn::{cmd::{DaemonCommand, DaemonCommandByte}, Bridge};

/// Struct which handles all parsing activity related to user input and data.
///
/// This struct is ZST, therefore it will be optimized by the compiler to only provide linking to
/// all it's related functions.
pub struct SugarParser;

impl SugarParser {
    /// Parses the daemon byte communication code and based on the result, calls different
    /// functions and changes the state of the communication bridge.
    pub fn parse_byte_code(bridge: &mut Bridge, command: DaemonCommand) -> ParseOutput {
        use DaemonCommandByte::*;
        let mut cmd_iter = command.byte_code().to_owned().into_iter(); // Creating an iterator of each byte.
        let mut size: u8 = 0;
        let mut count: u8 = 0;
        let mut prev_byte = None;

        loop {
            match cmd_iter.next() {
                Some(byte) => {
                    let _ = count.wrapping_add(byte.into()); 

                    // Parsing the byte itself.
                    match byte {
                        prefix @ (REQ | ACK | NACK) => {
                            prev_byte.replace(prefix);
                        },
                        data @ _ => {
                            // The first byte is always the size.
                            if prev_byte.is_none() {
                                size = data.into();
                                prev_byte.replace(DaemonCommandByte::SIZE);    
                            }

                            break; // Breaking out of the loop, because we cannot parse the code
                                   // that comes afterwards.
                        } 
                    }
                },
                // The parsing shall succeed only if the commands checksum is zero.
                None => return if count == 0 { 
                    if prev_byte.is_some() {
                        ParseOutput::Success
                    } else { ParseOutput::Empty }
                } else { ParseOutput::Checksum },
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
