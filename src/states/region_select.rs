use std::{
    env::{current_exe, VarError},
    num::{IntErrorKind, ParseIntError},
    sync::atomic::AtomicUsize,
    thread::current,
};

use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::Point,
    mono_font::ascii::*,
    mono_font::*,
    pixelcolor::Rgb888,
    prelude::{Dimensions, RgbColor},
    text::Alignment,
    text::Baseline,
    text::Text,
    Drawable,
};

use std::sync::atomic::{AtomicBool, Ordering};

use chrono_tz::Tz;
use chrono_tz::TZ_VARIANTS;
use std::sync::Mutex;

use crate::Matrix;
use crate::State;
use crate::State::*;

static TIMEZONE_INDEX: AtomicUsize = AtomicUsize::new(0);
static FRAME_COUNT: AtomicUsize = AtomicUsize::new(0);

enum RowType {
    REGULAR,
    HEADING,
    SELECTED,
}

// LAYOUT ---
// total height available: 32px
// heading: 1px padding top/bottom + 8px = 10px
// unselected text rows = 6px = 12px
// selected text row: 6px + 1px padding top/bottom = 8px

fn draw_menu_option(
    matrix: &mut Matrix,
    text: &str,
    row_num: usize,
    row_type: &RowType,
) -> Result<(), String> {
    use RowType::*;

    let font_regular: MonoTextStyle<Rgb888> =
        MonoTextStyle::new(&FONT_4X6, Rgb888::new(0xff, 0xff, 0xff));
    let font_selected: MonoTextStyle<Rgb888> =
        MonoTextStyle::new(&FONT_4X6, Rgb888::new(0x0, 0xff, 0x0));

    _ = Text::with_alignment(
        text,
        get_row_point(row_num as i32, row_type).unwrap(),
        match row_type {
            SELECTED => font_selected,
            _ => font_regular,
        },
        Alignment::Left,
    )
    .draw(matrix.get_canvas());

    Ok(())
}

const fn get_row_point(row_num: i32, row_type: &RowType) -> Result<Point, IntErrorKind> {
    use RowType::*;

    if row_num < 0 {
        return Err(IntErrorKind::InvalidDigit);
    }

    let padding = 1;
    let font_height = 6;
    return Ok(Point::new(
        match row_type {
            HEADING => 1,
            REGULAR => 5,
            SELECTED => 3,
        },
        font_height - 1
            + (row_num * (padding + font_height)
                + match row_num {
                    0 => 0,
                    _ => 2,
                }),
    ));
}

pub fn region_select_state(matrix: &mut Matrix) -> State {
    use RowType::*;

    let current_framecount = FRAME_COUNT.load(Ordering::Acquire);

    _ = draw_menu_option(matrix, "Region:", 0, &HEADING);

    _ = draw_menu_option(matrix, "Australia/Perth", 1, &REGULAR);

    _ = draw_menu_option(matrix, "Australia/Sydney", 2, &SELECTED);

    _ = draw_menu_option(matrix, "Australia/Melbourne", 3, &REGULAR);

    //for varient in TZ_VARIANTS {
    //    let name = varient.name();
    //}

    FRAME_COUNT.store(current_framecount + 1, Ordering::Release);

    return RegionSelect;
}
