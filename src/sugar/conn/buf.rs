
use rusb::{DeviceHandle, Error as RusbError};

/// Amount of bytes that will be held for user's commands input.
const INPUT_BUFFER_SIZE: usize = 128;
/// Amount of bytes that will be stored in the buffer from the target device.
const OUTPUT_BUFFER_SIZE: usize = 512;

/// Communication timeout for one atomic read/write in ms.
const TIMEOUT: std::time::Duration = std::time::Duration::from_millis(2000);

/// Custom trait for buffers.
pub(crate) trait Buffer: Send + Sync + 'static {
    /// Reads data from the bus and returns read bytes.
    fn read(&mut self, dev: &DeviceHandle<rusb::Context>) -> Result<&[u8], BufferError>;
    /// Writes data to the bus and returns the amount of bytes written.
    fn write(&mut self, dev: &DeviceHandle<rusb::Context>) -> Result<usize, BufferError>;
}

/// Buffer for USB v2.0.
///
/// Communication is done in half duplex, where writes are less common than reads. All
/// I/O is done via libusb protocol.
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
    fn read(&mut self, dev: &DeviceHandle<rusb::Context>) -> Result<&[u8], BufferError> {
        let ptr = self.read_ptr;
        let slice = &mut self._out[ptr..];

        // Reading with defined timeout.
        match dev.read_bulk(1, slice, TIMEOUT) {
            Ok(len) => {
                // Reading the latest data.
                let data = &self._out[ptr..ptr + len];

                // Moving the pointer forward without overflowing.
                self.read_ptr = (self.read_ptr + len) % OUTPUT_BUFFER_SIZE;

                Ok(data)
            }
            Err(err) => Err(BufferError::RusbError(err)),
        }
    }

    fn write(&mut self, dev: &DeviceHandle<rusb::Context>) -> Result<usize, BufferError> {
        let ptr = self.write_ptr;
        let offset = self._in[ptr] as usize;

        self._write(dev, ptr, offset)
    }
}

impl USBV2Buf {
    fn _write(
        &mut self,
        dev: &DeviceHandle<rusb::Context>,
        ptr: usize,
        offset: usize,
    ) -> Result<usize, BufferError> {
        let slice = &self._in[ptr + 1..ptr + 1 + offset];

        // Writing the slice in.
        match dev.write_bulk(1, slice, TIMEOUT) {
            Ok(len) => {
                // Moving the pointer forward without overflowing.
                self.write_ptr = (self.write_ptr + len) % INPUT_BUFFER_SIZE;

                Ok(len)
            }
            Err(err) => Err(BufferError::RusbError(err)),
        }
    }
}

/// Parsed error that can be obtained while doing operations on buffer.
#[derive(Debug)]
pub enum BufferError {
    NoData,
    BadData,
    RusbError(RusbError),
}

