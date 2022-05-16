//! Example Using linux_embedded_hal::I2cdev as the I2C implementation
use ezo_i2c_rs::EzoBoard;
use linux_embedded_hal::I2cdev;
use std::path::Path;
use std::thread;
use std::time::Duration;

const NINE_HUNDRED_MS_DELAY: Duration = Duration::from_millis(900);
const SIX_HUNDRED_MS_DELAY: Duration = Duration::from_millis(600);
const PH_ADDR: u8 = 0x63;
const EC_ADDR: u8 = 0x64;
const TEMP_ADDR: u8 = 0x66;

fn main() {
    let path = Path::new("/dev/i2c-1");
    let i2c_ph = I2cdev::new(path).unwrap();
    let i2c_ec = I2cdev::new(path).unwrap();
    let i2c_temp = I2cdev::new(path).unwrap();
    let mut ph_board = EzoBoard::new(i2c_ph, PH_ADDR);
    let mut ec_board = EzoBoard::new(i2c_ec, EC_ADDR);
    let mut temp_board = EzoBoard::new(i2c_temp, TEMP_ADDR);

    loop {
        temp_board.send_command(b"R", SIX_HUNDRED_MS_DELAY).unwrap();
        let temp = temp_board.read_response().unwrap();
        let prompt = format!("RT,{}", temp);
        ph_board
            .send_command(prompt.as_bytes(), NINE_HUNDRED_MS_DELAY)
            .unwrap();
        let ph = ph_board.read_response().unwrap();
        ec_board.send_command(b"R", SIX_HUNDRED_MS_DELAY).unwrap();
        let ec = ec_board.read_response().unwrap();

        println!("Temp: {} PH: {} EC: {}", temp, ph, ec);
        thread::sleep(Duration::from_secs(2));
    }
}
