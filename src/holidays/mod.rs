pub mod global;
pub mod united_states;

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
