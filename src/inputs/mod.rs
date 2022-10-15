pub mod bh1750;
pub mod joy_featherwing;

pub static DELAY_NS: usize = 100;

#[derive(Debug)]
pub enum InputError {
    LightReadErr,
    JoyReadErr,
    HwNotFound,
}

pub fn u32_to_u8s(input: u32) -> [u8; 4] {
    return [
        (input >> 24) as u8,
        (input >> 16) as u8,
        (input >> 8) as u8,
        input as u8,
    ];
}

pub fn u8s_to_u32(input: [u8; 4]) -> u32 {
    return ((input[0] << 24) | (input[1] << 16) | (input[2] << 8) | input[3]) as u32;
}
