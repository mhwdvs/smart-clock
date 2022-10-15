use crate::inputs::InputError;
use rppal::i2c::I2c;
use std::thread::sleep;
use std::time::Duration;

pub enum Button {
    None,
    Up,
    Down,
    Left,
    Right,
    Select,
}

// data sheet: https://cdn-learn.adafruit.com/downloads/pdf/adafruit-seesaw-atsamd09-breakout.pdf
// note: arduino must be read from in 32 byte chunks

static JOY_I2C_ADDR: u16 = 0x49;
static DELAY_MS: u64 = 10;

enum JoyInternalGPIOPins {
    ButtonRight = 0x06,
    ButtonDown = 0x07,
    ButtonLeft = 0x09,
    ButtonUp = 0x10,
    ButtonSelect = 0x14,
}

enum BaseRegister {
    STATUS = 0x00,
    GPIO = 0x01,
}

enum StatusFunctionRegister {
    HWID = 0x01,
    VERSION = 0x02,
    OPTIONS = 0x03,
    TEMP = 0x04,
    SWRST = 0x7f,
}

enum GPIOFunctionRegister {
    DIRSET = 0x02,
    DIRCLR = 0x03,
    GPIO = 0x04,
    SET = 0x05,
    CLR = 0x06,
    TOGGLE = 0x07,
    INTENSET = 0x08,
    INTENCLR = 0x09,
    INTFLAG = 0x0A,
    PULLENSET = 0x0B,
    PULLENCLR = 0x0C,
}

enum HardwareID {
    SAMD09 = 0x55,
    TINY8X7 = 0x87,
}

pub struct JoyFeatherwing {}

impl JoyFeatherwing {
    /// Resets all seesaw registers to their default values
    fn software_reset() {
        let mut channel = I2c::new().unwrap();
        channel.set_slave_address(JOY_I2C_ADDR);

        channel
            .write(&[
                BaseRegister::STATUS as u8,
                StatusFunctionRegister::SWRST as u8,
            ])
            .unwrap();

        sleep(Duration::from_millis(DELAY_MS));
    }

    /// Determines the seesaw's chipset
    fn hardware_id() -> Result<HardwareID, InputError> {
        let mut channel = I2c::new().unwrap();
        channel.set_slave_address(JOY_I2C_ADDR);

        channel
            .write(&[
                BaseRegister::STATUS as u8,
                StatusFunctionRegister::HWID as u8,
            ])
            .unwrap();

        sleep(Duration::from_millis(DELAY_MS));

        let mut buf: [u8; 1] = [0x0];
        let result_num = channel.read(&mut buf).unwrap();
        if result_num != 1 {
            return Err(InputError::JoyReadErr);
        }

        match buf[0] {
            x if x == HardwareID::SAMD09 as u8 => return Ok(HardwareID::SAMD09),
            x if x == HardwareID::TINY8X7 as u8 => return Ok(HardwareID::TINY8X7),
            _ => return Err(InputError::JoyReadErr),
        }
    }

    pub fn init() {
        // clean registers
        JoyFeatherwing::software_reset();

        println!("HWID: {}", JoyFeatherwing::hardware_id().unwrap() as u8);
        // pull-up buttons with PULLENSET

        // set GPIO interrupts
    }

    pub fn get_joy_buttons() -> Result<Button, InputError> {
        // digital read on button GPIO pins

        Ok(Button::None)
    }
}
