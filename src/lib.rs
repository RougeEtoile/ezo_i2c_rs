//! # ezo_i2c_rs
//!
//! `ezo_i2c_rs` provides a simple interface to interact with Atlas Scientific Ezo Devices, using the I2C protocol.
//!
//! Built on top of the I2C embedded-hal abstraction this library is platform agnostic. 

use std::thread;
use std::time::Duration;
use std::str;
use std::str::Utf8Error;
use std::result;
use thiserror::Error;
use embedded_hal as hal;
use hal::blocking::i2c;
use std::error::Error as StdError;

#[derive(Error, Debug)]
pub enum EzoBoardError<E: StdError + 'static> {
    /// Errors from I2C implementation
    #[error(transparent)]
    I2c(#[from] E),
    /// UTF8 Error
    #[error(transparent)]
    Utf8Error(Utf8Error),
    /// Read request returned Response Code 254
    #[error("Read buffer is not ready")]
    NotReady,
    /// Read request returned Response Code 255
    #[error("No Data To Send")]
    NoDataToSend,
    /// Read request returned Response Code 2
    #[error("Syntax Error")]
    SyntaxError,

    #[error("unknown EzoReadError")]
    Unknown,
}
///Newtype that encapsulates the I2C device and it's address.
pub struct EzoBoard<I2C> {
    i2c: I2C,
    address: u8
}
/// I2C must implement the embedded-hal traits for I2C. Specifically `Read` and `Write` are required.
///
/// https://docs.rs/embedded-hal/0.2.4/embedded_hal/blocking/i2c/index.html
impl<I2C, E> EzoBoard<I2C> 
    where I2C: i2c::Read<Error= E> + i2c::Write<Error = E>, E: std::error::Error {
    
    pub fn new(i2c:I2C, address: u8) -> Self {
        EzoBoard {
            i2c,
            address
        }
    }
    /// Sends a command to the Ezo device and then sleep the specified delay. Does not sleep on delay if delay is `Duration:new(0,0)`
    pub fn send_command(&mut self, command: &[u8], delay: Duration) -> result::Result<(), EzoBoardError<E>> {
        self.i2c.write(self.address, command)?;
        if delay != Duration::new(0, 0) {
            thread::sleep(delay);
        }
        Ok(())
    }
    /// Reads from ezo device, checks response code and returns the result as a string for convenience.
    pub fn read_response(&mut self) -> result::Result<String, EzoBoardError<E>> {
        let mut buff : [u8; 40] = [0; 40];
        self.i2c.read(self.address,&mut buff[..])?;
        match &buff[0] {
            1 => Ok(str::from_utf8(&buff[1..]).map_err(|e| EzoBoardError::Utf8Error(e))?.to_string()),
            2 => Err(EzoBoardError::SyntaxError),
            254 => Err(EzoBoardError::NotReady),
            255 => Err(EzoBoardError::NoDataToSend),
            _ => Err(EzoBoardError::Unknown)
        }
    }
}