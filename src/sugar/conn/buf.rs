//! Communication buffer implementation.

use hidapi::{HidDevice, HidError};

/// Amount of bytes that will be held for user's commands input.
const INPUT_BUFFER_SIZE: usize = 128;
/// Amount of bytes that will be stored in the buffer from the target device.
const OUTPUT_BUFFER_SIZE: usize = 512;

/// Communication timeout for one atomic read/write in ms.
const TIMEOUT: i32 = 2000;

/// Custom trait for buffers.
pub(crate) trait Buffer: Send + Sync + 'static { 
    /// Reads data from the bus and returns read bytes.
    fn read(&mut self, dev: &HidDevice) -> Result<&[u8], BufferError>;
    /// Writes data to the bus and returns the amount of bytes written.
    fn write(&mut self, dev: &HidDevice) -> Result<usize, BufferError>;
}

/// Buffer for USB v2.0.
///
/// Communication is done in half duplex, where writes are less common than reads. All
/// I/O is done via hidapi protocol.
#[derive(Debug)]
pub(crate) struct USBV2Buf {
    pub _in: [u8; INPUT_BUFFER_SIZE],
    pub _out: [u8; OUTPUT_BUFFER_SIZE],
    read_ptr: usize,
    write_ptr: usize,
}

impl Default for USBV2Buf {
    fn default() -> Self {
        Self {
            _in: [0u8; INPUT_BUFFER_SIZE],
            _out: [0u8; OUTPUT_BUFFER_SIZE],
            read_ptr: 0,
            write_ptr: 0,
        }
    }
}

impl Buffer for USBV2Buf {
    fn read(&mut self, dev: &HidDevice) -> Result<&[u8], BufferError> {
        use HidError::*;
        let ptr = self.read_ptr;
        let slice = self._out[ptr..].as_mut();

        // Reading with defined timeout.
        match dev.read_timeout(slice, TIMEOUT) {
            Ok(len) => {
                // Reading the latest data.
                let data = self._out[ptr .. ptr + len].as_ref();

                // Moving the pointer forward without overflowing.
                if self.read_ptr.saturating_add(len) > OUTPUT_BUFFER_SIZE {
                    self.read_ptr = 0;
                };

                Ok(data)
            }                             // Returning the amount of bytes read.
            Err(err) => Err(match err {
                HidApiErrorEmpty => BufferError::NoData,
                c @ _ => BufferError::HidAPI(c),            // Either impossible to parse at this
                                                            // level or unsolvable.
            }),
        }
    }

    fn write(&mut self, dev: &HidDevice) -> Result<usize, BufferError> {
        let ptr = self.write_ptr;
        let offset = self._in[ptr];

        self._write(dev, ptr, offset as usize)
    }
}

impl USBV2Buf {
    fn _write(&mut self, dev: &HidDevice, ptr: usize, offset: usize) -> Result<usize, BufferError> {
        use HidError::*;
        let slice = self._in[ptr .. ptr + offset].as_ref();

        // Writing the slice in.
        match dev.write(slice) {
            Ok(len) => {
                // Moving the pointer forward without overflowing.
                if self.write_ptr.saturating_add(len) > INPUT_BUFFER_SIZE {
                    self.write_ptr = 0;
                }; 

                Ok(len)
            },
            Err(err) => Err(match err {
                HidApiErrorEmpty => BufferError::NoData,
                InvalidZeroSizeData => BufferError::BadInput,   // Unable to write a ZST.
                // If a whole command was not sent, sending the rest.
                IncompleteSendError { sent, all } => return self._write(dev, ptr + sent, all - sent),
                c @ _ => BufferError::HidAPI(c),                // Either impossible to parse at this
                                                                // level or unsolvable.
            }),
        }
    }
}

/// Parsed error that can be obtained while doing operations on buffer.
#[derive(Debug)]
pub enum BufferError {
    NoData,
    BadInput,
    HidAPI(HidError),
}

