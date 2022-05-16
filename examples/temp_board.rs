//! Example Using linux_embedded_hal::I2cdev as the I2C implementation
use ezo_i2c_rs::EzoBoard;
use linux_embedded_hal::I2cdev;
use std::path::Path;
use std::thread;
use std::time::Duration;

const LONG_DELAY: Duration = Duration::from_millis(600);
const ADDR_TEMP: u8 = 0x66;

fn main() {
    let path = Path::new("/dev/i2c-1");
    let i2c_temp = I2cdev::new(path).unwrap();
    let mut temp_board = EzoBoard::new(i2c_temp, ADDR_TEMP);
    loop {
        temp_board.send_command(b"R", LONG_DELAY).unwrap();
        let temp = temp_board.read_response().unwrap();
        println!("Temp: {}", temp);
        thread::sleep(Duration::from_secs(2));
    }
}
