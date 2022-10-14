pub mod bh1750;
pub mod joy_featherwing;

pub static DELAY_NS: usize = 100;

pub enum InputError {
    LightReadErr,
    JoyReadErr,
    HwNotFound,
}
