use std::string::String;
use std::sync::Mutex;

pub mod region_select;
pub mod time;

lazy_static! {
    /// country, city, timezone
    static ref CURRENT_TIMEZONE: Mutex<chrono_tz::Tz> =
        Mutex::new(chrono_tz::TZ_VARIANTS[0]);
}
