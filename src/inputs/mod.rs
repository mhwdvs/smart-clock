mod bh1750;
mod joy_featherwing;

pub static DELAY_NS: usize = 100;

pub enum InputError {
    LightReadErr,
    JoyReadErr,
    HwNotFound,
}
