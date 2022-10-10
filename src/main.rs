//use embedded_graphics::pixelcolor::Rgb888;

mod matrix;
//#[cfg(not(all(target_arch = "arm", target_os = "linux", target_env = "gnu")))]
// mod mock_matrix;
//#[cfg(all(target_arch = "arm"))]
mod rpi_matrix;

pub fn main() {
    #[cfg(not(all(target_arch = "arm", target_os = "linux", target_env = "gnu")))]
    let mut framebuffer = matrix::get_matrix();

    #[cfg(all(target_arch = "arm", target_os = "linux", target_env = "gnu"))]
    let matrix = matrix::get_matrix();
    // establish frame buffer
    #[cfg(all(target_arch = "arm", target_os = "linux", target_env = "gnu"))]
    let mut framebuffer = matrix.offscreen_canvas();

    //let character_style = MonoTextStyle::new(&FONT_6X10, Rgb888::new(0xff, 0xff, 0xff));

    loop {
        //_ = Text::with_alignment(
        //    "10:52PM",
        //    display.bounding_box().center(),
        //    character_style,
        //    Alignment::Center,
        //)
        //.draw(&mut display);

        // swap frame buffer
        //#[cfg(all(target_arch = "arm", target_os = "linux", target_env = "gnu"))]
        //framebuffer = matrix.swap(canvas);
    }
}
