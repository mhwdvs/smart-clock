use embedded_graphics::{
    mock_display::MockDisplay,
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{
        Circle, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle, StrokeAlignment, Triangle,
    },
    text::{Alignment, Text},
};

pub fn get_matrix() -> MockDisplay<Rgb888> {
    let rect = Rectangle::new(Point::new(0, 0), Size::new(64, 32));
    return MockDisplay::from_points(rect.points(), Rgb888::new(0, 0, 0));
}
