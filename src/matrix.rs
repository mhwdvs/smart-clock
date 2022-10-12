#[cfg(not(all(target_arch = "arm", target_os = "linux", target_env = "gnu")))]
use embedded_graphics::{
    mock_display::MockDisplay, pixelcolor::Rgb888, prelude::*, primitives::Rectangle,
};

#[cfg(not(all(target_arch = "arm", target_os = "linux", target_env = "gnu")))]
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
};

#[cfg(all(target_arch = "arm"))]
use rpi_led_matrix::{LedCanvas, LedMatrix, LedMatrixOptions};

pub struct Matrix {
    #[cfg(all(target_arch = "arm"))]
    rpi_led_matrix: LedMatrix,
    #[cfg(all(target_arch = "arm"))]
    rpi_led_canvas: LedCanvas,

    #[cfg(not(all(target_arch = "arm", target_os = "linux", target_env = "gnu")))]
    sim_display: SimulatorDisplay<Rgb888>,
    #[cfg(not(all(target_arch = "arm", target_os = "linux", target_env = "gnu")))]
    sim_window: Window,
}

impl Matrix {
    #[cfg(all(target_arch = "arm", target_os = "linux", target_env = "gnu"))]
    pub fn new() -> Self {
        let mut options = LedMatrixOptions::new();
        _ = options.set_brightness(100);
        options.set_cols(64);
        options.set_rows(32);
        options.set_hardware_mapping("adafruit-hat-pwm");
        options.set_limit_refresh(30);

        let mut matrix = LedMatrix::new(Some(options), None).unwrap();

        Self {
            rpi_led_matrix: matrix,
            rpi_led_canvas: matrix.offscreen_canvas(),
        }
    }

    #[cfg(not(all(target_arch = "arm", target_os = "linux", target_env = "gnu")))]
    pub fn new() -> Self {
        let output_settings = OutputSettingsBuilder::new()
            .theme(BinaryColorTheme::Default)
            .build();

        let sim_display = SimulatorDisplay::new(Size::new(64, 32));

        let sim_window = Window::new("smart-clock", &output_settings);

        Self {
            sim_display,
            sim_window,
        }
    }

    #[cfg(all(target_arch = "arm", target_os = "linux", target_env = "gnu"))]
    pub fn get_canvas(&self) -> &mut LedCanvas {
        return &mut self.rpi_led_matrix.offscreen_canvas();
    }

    #[cfg(not(all(target_arch = "arm", target_os = "linux", target_env = "gnu")))]
    pub fn get_canvas(&mut self) -> &mut SimulatorDisplay<Rgb888> {
        return &mut self.sim_display;
    }

    #[cfg(all(target_arch = "arm", target_os = "linux", target_env = "gnu"))]
    pub fn swap_framebuffer(&mut self) {
        self.rpi_led_canvas = self.rpi_led_matrix.swap(self.rpi_led_canvas);
    }

    #[cfg(not(all(target_arch = "arm", target_os = "linux", target_env = "gnu")))]
    pub fn swap_framebuffer(&mut self) {
        self.sim_window.update(&self.sim_display);
    }
}
