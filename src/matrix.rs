#[cfg(not(all(target_arch = "arm", target_os = "linux", target_env = "gnu")))]
use embedded_graphics::{pixelcolor::Rgb888, prelude::*, primitives::Rectangle};

#[cfg(not(all(target_arch = "arm", target_os = "linux", target_env = "gnu")))]
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettings, OutputSettingsBuilder, SimulatorDisplay, Window,
};

#[cfg(all(target_arch = "arm"))]
use rpi_led_matrix::{LedCanvas, LedMatrix, LedMatrixOptions, LedRuntimeOptions};

#[cfg(all(target_arch = "arm"))]
use std::mem::swap;

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
        let mut matrix_options = LedMatrixOptions::new();
        _ = matrix_options.set_brightness(100);
        matrix_options.set_cols(64);
        matrix_options.set_rows(32);
        matrix_options.set_hardware_mapping("adafruit-hat-pwm");
        matrix_options.set_limit_refresh(0);
        matrix_options.set_led_rgb_sequence("rbg");

        let mut runtime_options = LedRuntimeOptions::new();
        //runtime_options.set_daemon(true);
        runtime_options.set_gpio_slowdown(2);

        let matrix = LedMatrix::new(Some(matrix_options), Some(runtime_options)).unwrap();
        let canvas = matrix.offscreen_canvas();

        Self {
            rpi_led_matrix: matrix,
            rpi_led_canvas: canvas,
        }
    }

    #[cfg(not(all(target_arch = "arm", target_os = "linux", target_env = "gnu")))]
    pub fn new() -> Self {
        let output_settings = OutputSettingsBuilder::new().scale(10).build();

        let sim_display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(Size::new(64, 32));

        let sim_window = Window::new("smart-clock", &output_settings);

        Self {
            sim_display,
            sim_window,
        }
    }

    #[cfg(all(target_arch = "arm", target_os = "linux", target_env = "gnu"))]
    pub fn get_canvas(&mut self) -> &mut LedCanvas {
        return &mut self.rpi_led_canvas;
    }

    #[cfg(not(all(target_arch = "arm", target_os = "linux", target_env = "gnu")))]
    pub fn get_canvas(&mut self) -> &mut SimulatorDisplay<Rgb888> {
        return &mut self.sim_display;
    }

    #[cfg(all(target_arch = "arm", target_os = "linux", target_env = "gnu"))]
    pub fn pre_draw(&mut self) {
        // crate::rpi_led_matrix shadows DrawTarget clear() implementation
        _ = self.rpi_led_canvas.clear();
    }

    #[cfg(not(all(target_arch = "arm", target_os = "linux", target_env = "gnu")))]
    pub fn pre_draw(&mut self) {
        _ = self.sim_display.clear(Rgb888::BLACK);
    }

    #[cfg(all(target_arch = "arm", target_os = "linux", target_env = "gnu"))]
    pub fn post_draw(mut self) -> Self {
        self.rpi_led_canvas = self.rpi_led_matrix.swap(self.rpi_led_canvas);
        return self;
    }

    #[cfg(not(all(target_arch = "arm", target_os = "linux", target_env = "gnu")))]
    pub fn post_draw(mut self) -> Self {
        self.sim_window.update(&self.sim_display);
        return self;
    }
}
