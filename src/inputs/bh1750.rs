use crate::inputs::InputError;
use rppal::i2c::I2c;

static BH1750_ADDR: u8 = 0x23;

// instruction set: https://www.mouser.com/datasheet/2/348/bh1750fvi-e-186247.pdf
enum Command {
    PowerOff = 0x0,
    PowerOn = 0x1,
    ResetMeasurement = 0x7,
    QualityHigh = 0x20,
    QualityHigh2 = 0x21,
    QualityLow = 0x23,
}

pub struct BH1750 {}

impl BH1750 {
    pub fn test_light_conn() -> Result<(), InputError> {
        let mut channel = I2c::new()?;
        channel.set_slave_address(BH1750_ADDR);
        channel.with_bus(1);
        //channel.smbus_read_byte();

        Ok(())
    }

    pub fn get_light() -> Result<(), InputError> {
        let mut channel = I2c::new()?;
        channel.set_slave_address(BH1750_ADDR);
        channel.with_bus(1);

        let buf: [u8; 2];
        let result = channel.read(buf).unwrap();
        if result != 2 {
            return Err(InputError::LightReadErr);
        }

        println!("{}, {}", buf[0], buf[1]);

        Ok(())
    }
}
