//! A selection of pre-defined holidays provided for convenience

pub mod global;
pub mod united_states;

use crate::Holiday;
use std::str::FromStr;

#[macro_export]
/// Macro to create Holiday
macro_rules! holiday {
    ($name:expr, $month:expr, $day:expr) => {
        Holiday {
            name: $name,
            date: HolidayDate::FixedDate(DayOfMonth {
                day: $day,
                month: $month,
            }),
        }
    };

    ($name:expr, $nth:expr, $weekday:expr, $month:expr) => {
        Holiday {
            name: $name,
            date: HolidayDate::NthDate(NthWeekdayOfMonth {
                nth: $nth,
                weekday: $weekday,
                month: $month,
            }),
        }
    };
}

#[macro_export]
/// Macro to create a `pub const Holiday`
macro_rules! holiday_const {
    ($(#[$attr:meta])* $var:ident, $name:expr, $month:expr, $day:expr) => {
        $(#[$attr])*
        pub const $var: Holiday<&str> = holiday!($name, $month, $day);
    };

    ($(#[$attr:meta])* $var:ident, $name:expr, $nth:expr, $weekday:expr, $month:expr) => {
        $(#[$attr])*
        pub const $var: Holiday<&str> = holiday!($name, $nth, $weekday, $month);
    };
}

impl FromStr for Holiday<&str> {
    type Err = std::io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use united_states::*;
        use global::*;
        Ok(match s.to_lowercase().as_str() {
            "martin luther king jr. day" => MLKJ_DAY,
            "groundhog day" => GROUNDHOG_DAY,
            "super bowl sunday" => SUPERBOWL_SUNDAY,
            "president's day" => PRESIDENTS_DAY,
            "valentine's day" => VALENTINES_DAY,
            "daylight saving time starts" => DST_START,
            "april fool's day" => APRIL_FOOLS_DAY,
            "kentucky derby" => KENTUCKY_DERBY,
            "memorial day" => MEMORIAL_DAY,
            "mother's day" => MOTHERS_DAY,
            "flag day" => FLAG_DAY,
            "independence day" | "july 4th" | "july fourth" | "fourth of july" => INDEPENDENCE_DAY,
            "father's day" => FATHERS_DAY,
            "labor day" => LABOR_DAY,
            "halloween" => HALLOWEEN,
            "columbus day" => COLUMBUS_DAY,
            "veteran's day" => VETERANS_DAY,
            "daylight saving time ends" => DST_END,
            "thanksgiving" => THANKSGIVING,
            "christmas" => CHRISTMAS,
            _ => return Err(std::io::Error::from(std::io::ErrorKind::InvalidInput))
        })
    }
}
