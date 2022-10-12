use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::Point,
    mono_font::ascii::*,
    mono_font::*,
    pixelcolor::Rgb888,
    prelude::{Dimensions, RgbColor},
    text::Alignment,
    text::Text,
    Drawable,
};

mod matrix;
use matrix::Matrix;

pub fn main() {
    let mut matrix = Matrix::new();

    let character_style = MonoTextStyle::new(&FONT_6X10, Rgb888::new(0xff, 0xff, 0xff));

    let p_tl = Point::new(0, 0);
    let p_ml = Point::new(0, 16);
    let p_center = matrix.get_canvas().bounding_box().center();

    let mut myint = 1;

    loop {
        matrix.pre_draw();

        _ = Text::with_alignment(
            myint.to_string().as_str(),
            p_center,
            character_style,
            Alignment::Center,
        )
        .draw(matrix.get_canvas());

        myint += 1;

        matrix = matrix.post_draw();
    }
}
