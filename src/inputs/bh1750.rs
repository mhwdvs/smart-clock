use crate::inputs::InputError;
use rppal::i2c::I2c;

// data sheet: https://www.mouser.com/datasheet/2/348/bh1750fvi-e-186247.pdf

static BH1750_ADDR: u16 = 0x23;
static MEASUREMENT_DELAY_MS: usize = 150;

mod Command {
    pub static PowerOff: u8 = 0x0;
    pub static PowerOn: u8 = 0x1;
    pub static ResetMeasurement: u8 = 0x7;
    pub static QualityHigh: u8 = 0x20;
    pub static QualityHigh2: u8 = 0x21;
    pub static QualityLow: u8 = 0x23;
}

pub struct BH1750 {}

impl BH1750 {
    pub fn init() -> Result<(), InputError> {
        let mut channel = I2c::new().unwrap();
        channel.set_slave_address(BH1750_ADDR);

        channel.write(&[Command::PowerOn as u8]).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(
            MEASUREMENT_DELAY_MS as u64,
        ));

        Ok(())
    }

    pub fn get_light() -> Result<(), InputError> {
        let mut channel = I2c::new().unwrap();
        channel.set_slave_address(BH1750_ADDR);

        channel.write(&[Command::QualityHigh2]).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(
            MEASUREMENT_DELAY_MS as u64,
        ));

        let mut buf: [u8; 2] = [0x0, 0x0];
        let result = channel.read(&mut buf).unwrap();
        if result != 2 {
            return Err(InputError::LightReadErr);
        }

        println!("{}, {}", buf[0], buf[1]);

        Ok(())
    }
}
