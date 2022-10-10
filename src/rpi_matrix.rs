use rpi_led_matrix::{LedMatrix, LedMatrixOptions};

pub fn get_matrix() -> LedMatrix {
    let mut options = LedMatrixOptions::new();
    _ = options.set_brightness(100);
    options.set_cols(64);
    options.set_rows(32);
    options.set_hardware_mapping("adafruit-hat-pwm");
    options.set_limit_refresh(30);

    return LedMatrix::new(Some(options), None).unwrap();
}
