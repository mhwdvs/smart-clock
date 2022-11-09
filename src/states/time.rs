use std::time::{SystemTime, UNIX_EPOCH};

use chrono::TimeZone;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::{
    geometry::Point, mono_font::ascii::*, mono_font::*, pixelcolor::Rgb888, text::Alignment,
    text::Text, Drawable,
};

use crate::Matrix;
use crate::State;
use crate::State::*;

use super::CURRENT_TIMEZONE;

fn draw_time(matrix: &mut Matrix) {
    let font_red: MonoTextStyle<Rgb888> =
        MonoTextStyle::new(&FONT_8X13, Rgb888::new(0xff, 0x0, 0x0));

    // get current UNIX Epoch time

    let unix_epoch_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let local_time = CURRENT_TIMEZONE
        .lock()
        .unwrap()
        .timestamp(unix_epoch_time.try_into().unwrap(), 0);

    _ = Text::with_alignment(
        local_time.format("%H:%M:%S").to_string().as_str(),
        Point::new(1, 15),
        font_red,
        Alignment::Left,
    )
    .draw(matrix.get_canvas());
}

pub fn time_state(matrix: &mut Matrix) -> State {
    draw_time(matrix);

    return Time;
}
