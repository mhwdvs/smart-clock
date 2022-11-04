use std::{num::IntErrorKind, sync::atomic::AtomicUsize};

use embedded_graphics::{
    geometry::Point, mono_font::ascii::*, mono_font::*, pixelcolor::Rgb888, text::Alignment,
    text::Text, Drawable,
};

use std::sync::atomic::Ordering;

use chrono_tz::TZ_VARIANTS;

use crate::Button;
use crate::JoyFeatherwing;
use crate::Matrix;
use crate::State;

static TIMEZONE_INDEX: AtomicUsize = AtomicUsize::new(0);
static FRAME_COUNT: AtomicUsize = AtomicUsize::new(0);
static FRAMES_SINCE_LAST_INPUT_POLL: AtomicUsize = AtomicUsize::new(0);

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
    use State::*;

    let current_framecount = FRAME_COUNT.load(Ordering::Acquire);
    let mut current_timezone_index = TIMEZONE_INDEX.load(Ordering::Acquire);
    let mut current_frames_since_last_input_poll =
        FRAMES_SINCE_LAST_INPUT_POLL.load(Ordering::Acquire);

    let input_poll_interval: usize = 5;

    println!("{}", current_frames_since_last_input_poll);

    if current_frames_since_last_input_poll == input_poll_interval {
        let buttons = JoyFeatherwing::get_joy_buttons();
        for button in buttons {
            match button {
                Button::Down => {
                    if current_timezone_index < TZ_VARIANTS.len() - 1 {
                        current_timezone_index += 1;
                    }
                }
                Button::Left => println!("Left"),
                Button::Right => println!("Right"),
                Button::Up => {
                    if current_timezone_index != 0 {
                        current_timezone_index -= 1
                    }
                }
                Button::Select => println!("Select"),
                _ => {}
            }
        }
        current_frames_since_last_input_poll = 0;
    } else {
        current_frames_since_last_input_poll += 1;
    }

    _ = draw_menu_option(matrix, "Region:", 0, &HEADING);
    if current_timezone_index == 0 {
        _ = draw_menu_option(
            matrix,
            TZ_VARIANTS[current_timezone_index].name(),
            1,
            &SELECTED,
        );
        _ = draw_menu_option(
            matrix,
            TZ_VARIANTS[current_timezone_index + 1].name(),
            2,
            &REGULAR,
        );
        _ = draw_menu_option(
            matrix,
            TZ_VARIANTS[current_timezone_index + 2].name(),
            3,
            &REGULAR,
        );
    } else if current_timezone_index == TZ_VARIANTS.len() - 1 {
        _ = draw_menu_option(
            matrix,
            TZ_VARIANTS[current_timezone_index - 3].name(),
            1,
            &REGULAR,
        );
        _ = draw_menu_option(
            matrix,
            TZ_VARIANTS[current_timezone_index - 2].name(),
            2,
            &REGULAR,
        );
        _ = draw_menu_option(
            matrix,
            TZ_VARIANTS[current_timezone_index - 1].name(),
            3,
            &SELECTED,
        );
    } else {
        _ = draw_menu_option(
            matrix,
            TZ_VARIANTS[current_timezone_index - 1].name(),
            1,
            &REGULAR,
        );
        _ = draw_menu_option(
            matrix,
            TZ_VARIANTS[current_timezone_index].name(),
            2,
            &SELECTED,
        );
        _ = draw_menu_option(
            matrix,
            TZ_VARIANTS[current_timezone_index + 1].name(),
            3,
            &REGULAR,
        );
    }

    FRAME_COUNT.store(current_framecount + 1, Ordering::Release);
    TIMEZONE_INDEX.store(current_timezone_index, Ordering::Release);
    TIMEZONE_INDEX.store(current_frames_since_last_input_poll, Ordering::Release);

    return RegionSelect;
}
