use std::collections::HashSet;
use std::num::IntErrorKind;
use std::sync::Mutex;

use chrono_tz::Tz;
use embedded_graphics::{
    geometry::Point, mono_font::ascii::*, mono_font::*, pixelcolor::Rgb888, text::Alignment,
    text::Text, Drawable,
};

use std::collections::BTreeMap;

use chrono_tz::TZ_VARIANTS;

use crate::states::CURRENT_TIMEZONE;
use crate::Button;
use crate::JoyFeatherwing;
use crate::Matrix;
use crate::State;

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

/// country, city, TZ
fn get_countries() -> Vec<&'static str> {
    let mut countries: HashSet<&str>;

    let mut i = 0;
    while i < TZ_VARIANTS.len() {
        let collection: Vec<&str> = TZ_VARIANTS[i].name().split("/").collect();
        match collection.get(0) {
            Some(x) => {
                countries.insert(x);
            }
            None => {}
        };
    }

    countries.into_iter().collect()
}

fn get_cities(country: &str) -> Vec<&'static str> {
    let mut cities: HashSet<&str>;

    let mut i = 0;
    while i < TZ_VARIANTS.len() {
        let collection: Vec<&str> = TZ_VARIANTS[i].name().split("/").collect();
        let a = String::from(country);
        let b = String::from(*collection.get(0).unwrap());
        if a == b {
            if collection.get(1).is_some() {
                cities.insert(collection.get(1).unwrap());
            }
        };
    }

    cities.into_iter().collect()
}

fn get_timezone(country: &str, city: &str) -> Option<chrono_tz::Tz> {
    let mut i = 0;
    while i < TZ_VARIANTS.len() {
        let collection: Vec<&str> = TZ_VARIANTS[i].name().split("/").collect();
        let a = String::from(country);
        let b = String::from(*collection.get(0).unwrap());
        if a == b {
            if collection.get(1).is_some() {
                let x = String::from(*collection.get(1).unwrap());
                let y = String::from(city);
                if x == y {
                    return Some(TZ_VARIANTS[i]);
                }
            }
        };
    }

    None
}

lazy_static! {
    static ref FRAME_COUNT: Mutex<usize> = Mutex::new(0);
    static ref FRAMES_SINCE_LAST_INPUT_POLL: Mutex<usize> = Mutex::new(0);
    static ref COUNTRY_INDEX: Mutex<usize> = Mutex::new(0);
    static ref COUNTRY_SELECTED: Mutex<bool> = Mutex::new(false);
    static ref CITY_INDEX: Mutex<usize> = Mutex::new(0);
}

pub fn region_select_state(matrix: &mut Matrix) -> State {
    use RowType::*;
    use State::*;

    // acquire locks on state variables
    let current_framecount = FRAME_COUNT.lock().unwrap();
    let mut current_frames_since_last_input_poll = FRAMES_SINCE_LAST_INPUT_POLL.lock().unwrap();

    let mut country_index = COUNTRY_INDEX.lock().unwrap();
    let mut country_selected = COUNTRY_SELECTED.lock().unwrap();
    let mut city_index = CITY_INDEX.lock().unwrap();

    let input_poll_interval: usize = 7;

    let filtered = TZ_VARIANTS
        .iter()
        .filter(|&x| {
            x.to_string().split('/');
            true
        })
        .collect::<Vec<&chrono_tz::Tz>>();

    for asd in filtered {}

    let current_country = get_countries()[*country_index];
    let current_city = get_cities(current_country)[*city_index];

    if *current_frames_since_last_input_poll == input_poll_interval {
        let buttons = JoyFeatherwing::get_joy_buttons();
        for button in buttons {
            match button {
                Button::Down => {
                    if *country_selected {
                        // test for city down
                        if *city_index < get_cities(current_country).len() {
                            *city_index += 1;
                        }
                    } else {
                        // test for country down
                        if *country_index < get_countries().len() {
                            *country_index += 1;
                        }
                    }
                }
                Button::Left => {
                    // back out of menu if possible
                    if *country_selected {
                        *country_selected = false;
                    }
                }
                Button::Right => {
                    // select country or city
                    if *country_selected {
                        *CURRENT_TIMEZONE.lock().unwrap() =
                            get_timezone(current_country, current_city).unwrap();
                        return State::Time;
                    } else {
                        *country_selected = true;
                    }
                }
                Button::Up => {
                    if *country_selected {
                        // test for city up
                        if *city_index != 0 {
                            *city_index -= 1;
                        }
                    } else {
                        // test for country up
                        if *country_index != 0 {
                            *country_index -= 1;
                        }
                    }
                }
                _ => {}
            }
        }
        *current_frames_since_last_input_poll = 0;
    }

    // heading
    _ = draw_menu_option(matrix, "Region:", 0, &HEADING);

    // options
    for country in get_countries().iter() {}

    if !*country_selected {
        // provide country options
        if *country_index == 0 {
            _ = draw_menu_option(matrix, get_countries()[*country_index], 1, &SELECTED);
            _ = draw_menu_option(matrix, get_countries()[*country_index + 1], 2, &REGULAR);
            _ = draw_menu_option(matrix, get_countries()[*country_index + 2], 3, &REGULAR);
        } else if *country_index == get_countries().len() - 1 {
            _ = draw_menu_option(matrix, get_countries()[*country_index - 2], 1, &REGULAR);
            _ = draw_menu_option(matrix, get_countries()[*country_index - 1], 2, &REGULAR);
            _ = draw_menu_option(matrix, get_countries()[*country_index], 3, &SELECTED);
        } else {
            _ = draw_menu_option(matrix, get_countries()[*country_index - 1], 1, &REGULAR);
            _ = draw_menu_option(matrix, get_countries()[*country_index], 2, &REGULAR);
            _ = draw_menu_option(matrix, get_countries()[*country_index + 1], 3, &REGULAR);
        }
    } else {
        // provide city options
        if *city_index == 0 {
            _ = draw_menu_option(
                matrix,
                get_cities(current_country)[*city_index],
                1,
                &SELECTED,
            );
            _ = draw_menu_option(
                matrix,
                get_cities(current_country)[*city_index + 1],
                2,
                &REGULAR,
            );
            _ = draw_menu_option(
                matrix,
                get_cities(current_country)[*city_index + 2],
                3,
                &REGULAR,
            );
        } else if *city_index == get_cities(current_country).len() - 1 {
            _ = draw_menu_option(
                matrix,
                get_cities(current_country)[*city_index - 2],
                1,
                &REGULAR,
            );
            _ = draw_menu_option(
                matrix,
                get_cities(current_country)[*city_index - 1],
                2,
                &REGULAR,
            );
            _ = draw_menu_option(
                matrix,
                get_cities(current_country)[*city_index],
                3,
                &SELECTED,
            );
        } else {
            _ = draw_menu_option(
                matrix,
                get_cities(current_country)[*city_index - 1],
                1,
                &REGULAR,
            );
            _ = draw_menu_option(
                matrix,
                get_cities(current_country)[*city_index],
                2,
                &REGULAR,
            );
            _ = draw_menu_option(
                matrix,
                get_cities(current_country)[*city_index + 1],
                3,
                &REGULAR,
            );
        }
    }

    return RegionSelect;
}
