use crate::inputs::InputError;
use rppal::i2c::I2C;

static JOY_FEATHERWING_ADDR: u8 = 0x49;

pub enum Button {
    None,
    Up,
    Down,
    Left,
    Right,
    Select,
}

pub struct JoyFeatherwing {}

impl JoyFeatherwing {
    pub fn test_joy_conn() -> Result<(), InputError> {
        Ok(())
    }

    pub fn get_joy_buttons() -> Result<Button, InputError> {
        Ok(Button::None)
    }
}
