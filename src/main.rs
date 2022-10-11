use embedded_graphics::{
    draw_target::DrawTarget, mono_font::ascii::*, mono_font::*, pixelcolor::Rgb888,
    prelude::Dimensions, text::Alignment, text::Text, Drawable,
};

mod matrix;
use matrix::Matrix;

pub fn main() {
    let matrix = Matrix::new();

    let character_style = MonoTextStyle::new(&FONT_6X10, Rgb888::new(0xff, 0xff, 0xff));

    loop {
        let mut canvas = matrix.get_canvas();
        let p_center = canvas.bounding_box().center();

        _ = Text::with_alignment("10:52PM", p_center, character_style, Alignment::Center)
            .draw(&mut canvas);

        // swap frame buffer
        matrix.swap_framebuffer();
    }
}
