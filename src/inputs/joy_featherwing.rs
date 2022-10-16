use crate::inputs::InputError;
use rppal::i2c::I2c;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::thread::sleep;
use std::time::Duration;

use crate::inputs::u32_to_u8s;
use crate::inputs::u8s_to_u32;

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
    /// ButtonRight = 6,
    ButtonA = 30,
    /// ButtonDown = 7,
    ButtonB = 31,
    /// ButtonLeft = 9,
    ButtonY = 18,
    /// ButtonUp = 10,
    ButtonX = 17,
    /// ButtonSelect = 14
    ButtonSelect = 22,
}

static JOY_BUTTON_PIN_BITMASK: [u32; 1] = [(1 << JoyInternalGPIOPins::ButtonA as u8)
    | (1 << JoyInternalGPIOPins::ButtonB as u8)
    | (1 << JoyInternalGPIOPins::ButtonX as u8)
    | (1 << JoyInternalGPIOPins::ButtonY as u8)
    | (1 << JoyInternalGPIOPins::ButtonSelect as u8)];

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

static BUTTON_A_PRESSED: AtomicBool = AtomicBool::new(false);
static BUTTON_B_PRESSED: AtomicBool = AtomicBool::new(false);
static BUTTON_X_PRESSED: AtomicBool = AtomicBool::new(false);
static BUTTON_Y_PRESSED: AtomicBool = AtomicBool::new(false);
static BUTTON_SELECT_PRESSED: AtomicBool = AtomicBool::new(false);

pub struct JoyFeatherwing {}

impl JoyFeatherwing {
    /// Resets all seesaw registers to their default values
    fn software_reset() {
        let mut channel = I2c::new().unwrap();
        channel.set_slave_address(JOY_I2C_ADDR);

        let mut written = false;
        while !written {
            if let Ok(result) = channel.write(&[
                BaseRegister::STATUS as u8,
                StatusFunctionRegister::SWRST as u8,
                0xFF, // no idea what this is
            ]) {
                written = true;
            }
        }

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

    fn pullup_pins() -> Result<(), InputError> {
        let mut channel = I2c::new().unwrap();
        channel.set_slave_address(JOY_I2C_ADDR);

        let JOY_ALL_PINS_BITMASK: [u32; 1] = [{
            let mut bitmask: u32 = 0;
            for i in 0u8..32u8 {
                bitmask |= 1 << i;
            }
            bitmask
        }];

        // dirclr - set pins to INPUT
        channel
            .write(&{
                let left = [BaseRegister::GPIO as u8, GPIOFunctionRegister::DIRCLR as u8];
                let right = u32_to_u8s(&JOY_BUTTON_PIN_BITMASK);
                let whole: [u8; 6] = {
                    let mut whole: [u8; 6] = [0; 6];
                    let (one, two) = whole.split_at_mut(left.len());
                    one.copy_from_slice(&left);
                    two.copy_from_slice(&right);
                    whole
                };
                whole
            })
            .unwrap();
        sleep(Duration::from_millis(DELAY_MS));

        // pullenset - enables PULLUP/PULLDOWN depending on high/low
        channel
            .write(&{
                let left = [
                    BaseRegister::GPIO as u8,
                    GPIOFunctionRegister::PULLENSET as u8,
                ];
                let right = u32_to_u8s(&JOY_BUTTON_PIN_BITMASK);
                let whole: [u8; 6] = {
                    let mut whole: [u8; 6] = [0; 6];
                    let (one, two) = whole.split_at_mut(left.len());
                    one.copy_from_slice(&left);
                    two.copy_from_slice(&right);
                    whole
                };
                whole
            })
            .unwrap();
        sleep(Duration::from_millis(DELAY_MS));

        // set - set pins to HIGH
        // xclr - set pins to LOW
        channel
            .write(&{
                let left = [BaseRegister::GPIO as u8, GPIOFunctionRegister::SET as u8];
                let right = u32_to_u8s(&JOY_BUTTON_PIN_BITMASK);
                let whole: [u8; 6] = {
                    let mut whole: [u8; 6] = [0; 6];
                    let (one, two) = whole.split_at_mut(left.len());
                    one.copy_from_slice(&left);
                    two.copy_from_slice(&right);
                    whole
                };
                whole
            })
            .unwrap();
        sleep(Duration::from_millis(DELAY_MS));

        Ok(())
    }

    fn set_GPIO_interupts() -> Result<(), InputError> {
        let mut channel = I2c::new().unwrap();
        channel.set_slave_address(JOY_I2C_ADDR);

        // intenset
        channel
            .write(&{
                let left = [
                    BaseRegister::GPIO as u8,
                    GPIOFunctionRegister::INTENSET as u8,
                ];
                let right = u32_to_u8s(&JOY_BUTTON_PIN_BITMASK);
                let whole: [u8; 6] = {
                    let mut whole: [u8; 6] = [0; 6];
                    let (one, two) = whole.split_at_mut(left.len());
                    one.copy_from_slice(&left);
                    two.copy_from_slice(&right);
                    whole
                };
                whole
            })
            .unwrap();
        sleep(Duration::from_millis(DELAY_MS));

        Ok(())
    }

    pub fn init() {
        // clean registers
        JoyFeatherwing::software_reset();

        // check that featherwing returns valid hardware id
        _ = JoyFeatherwing::hardware_id().unwrap();

        // pull-up buttons with PULLENSET
        _ = JoyFeatherwing::pullup_pins().unwrap();

        // set GPIO interrupts
        //_ = JoyFeatherwing::set_GPIO_interupts().unwrap();
    }

    pub fn measure_joy_buttons() {
        let mut channel = I2c::new().unwrap();
        channel.set_slave_address(JOY_I2C_ADDR);

        let JOY_ALL_PINS_BITMASK: [u32; 1] = [{
            let mut bitmask: u32 = 0;
            for i in 0u8..32u8 {
                bitmask |= 1 << i;
            }
            bitmask
        }];

        // digital read on button GPIO pins
        if let Err(r) = channel.write(&[BaseRegister::GPIO as u8, GPIOFunctionRegister::GPIO as u8])
        {
            JoyFeatherwing::init();
            return; // abort silently
        }
        sleep(Duration::from_millis(DELAY_MS));

        let mut buf: [u8; 4] = [0x0; 4];
        match channel.read(&mut buf) {
            Ok(result_num) => {
                if result_num != 4 {
                    return; // abort silently
                }
            }
            Err(err) => {
                JoyFeatherwing::init();
                return; // abort silently
            }
        }
        let buf32 = u8s_to_u32(&buf)[0];

        let res = !(JOY_BUTTON_PIN_BITMASK[0] & buf32);

        if (res & (1u32 << JoyInternalGPIOPins::ButtonA as u32)) != 0 {
            BUTTON_A_PRESSED.store(true, Ordering::Relaxed);
        } else {
            BUTTON_A_PRESSED.store(false, Ordering::Relaxed);
        }

        if (res & (1u32 << JoyInternalGPIOPins::ButtonB as u32)) != 0 {
            BUTTON_B_PRESSED.store(true, Ordering::Relaxed);
        } else {
            BUTTON_B_PRESSED.store(false, Ordering::Relaxed);
        }

        if (res & (1u32 << JoyInternalGPIOPins::ButtonX as u32)) != 0 {
            BUTTON_X_PRESSED.store(true, Ordering::Relaxed);
        } else {
            BUTTON_X_PRESSED.store(false, Ordering::Relaxed);
        }

        if (res & (1u32 << JoyInternalGPIOPins::ButtonY as u32)) != 0 {
            BUTTON_Y_PRESSED.store(true, Ordering::Relaxed);
        } else {
            BUTTON_Y_PRESSED.store(false, Ordering::Relaxed);
        }

        if (res & (1u32 << JoyInternalGPIOPins::ButtonSelect as u32)) != 0 {
            BUTTON_SELECT_PRESSED.store(true, Ordering::Relaxed);
        } else {
            BUTTON_SELECT_PRESSED.store(false, Ordering::Relaxed);
        }
    }

    pub fn get_joy_buttons() -> Vec<Button> {
        let mut buttons: Vec<Button> = Vec::new();

        if BUTTON_A_PRESSED.load(Ordering::Relaxed) {
            buttons.push(Button::Up);
        }
        if BUTTON_B_PRESSED.load(Ordering::Relaxed) {
            buttons.push(Button::Down);
        }
        if BUTTON_X_PRESSED.load(Ordering::Relaxed) {
            buttons.push(Button::Left);
        }
        if BUTTON_Y_PRESSED.load(Ordering::Relaxed) {
            buttons.push(Button::Right);
        }
        if BUTTON_SELECT_PRESSED.load(Ordering::Relaxed) {
            buttons.push(Button::Select);
        }

        buttons
    }
}
