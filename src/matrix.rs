//#[cfg(not(all(target_arch = "arm", target_os = "linux", target_env = "gnu")))]
//use {
//    crate::mock_matrix,
//    embedded_graphics::{mock_display::MockDisplay, pixelcolor::Rgb888},
//};
//#[cfg(all(target_arch = "arm"))]
use {crate::rpi_matrix, rpi_led_matrix::LedMatrix};

//#[cfg(all(target_arch = "arm", target_os = "linux", target_env = "gnu"))]
pub fn get_matrix() -> LedMatrix {
    return rpi_matrix::get_matrix();
}

//#[cfg(not(all(target_arch = "arm", target_os = "linux", target_env = "gnu")))]
//pub fn get_matrix() -> MockDisplay<Rgb888> {
//    return mock_matrix::get_matrix();
//}
