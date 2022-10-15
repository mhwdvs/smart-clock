pub mod bh1750;
pub mod joy_featherwing;

pub static DELAY_NS: usize = 100;

#[derive(Debug)]
pub enum InputError {
    LightReadErr,
    JoyReadErr,
    HwNotFound,
}
