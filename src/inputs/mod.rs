pub mod bh1750;
pub mod joy_featherwing;

#[derive(Debug)]
#[allow(dead_code)]
pub enum InputError {
    ReadErr,
    WriteErr,
    HwNotFound,
}

fn u32_to_u8s(arr: &[u32]) -> &[u8] {
    let len = 4 * arr.len();
    let ptr = arr.as_ptr() as *const u8;
    unsafe { std::slice::from_raw_parts(ptr, len) }
}

pub fn u8s_to_u32(arr: &[u8]) -> &[u32] {
    let len = arr.len() / 4;
    let ptr = arr.as_ptr() as *const u32;
    unsafe { std::slice::from_raw_parts(ptr, len) }
}
