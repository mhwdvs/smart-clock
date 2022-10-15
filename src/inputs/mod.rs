pub mod bh1750;
pub mod joy_featherwing;

pub static DELAY_NS: usize = 100;

#[derive(Debug)]
pub enum InputError {
    LightReadErr,
    JoyReadErr,
    HwNotFound,
}

fn u32_to_u8s(arr: &[u32]) -> &[u8] {
    let len = 4 * arr.len();
    let ptr = arr.as_ptr() as *const u8;
    unsafe { std::slice::from_raw_parts(ptr, len) }
}

pub fn u8s_to_u32(input: [u8; 4]) -> u32 {
    return ((input[0] << 24) | (input[1] << 16) | (input[2] << 8) | input[3]) as u32;
}
