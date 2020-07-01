#![warn(missing_docs)]

//! # holiday
//!
//! A library for defining annually repeating dates and holidays
//!
//! ```rust
//! use holiday::*;
//! use holiday::holidays::united_states::THANKSGIVING;
//! use chrono::{Weekday, NaiveDate};
//!
//! // Pastover: First Friday in April
//! let pastover = Holiday::new_nth("Pastover", 1, Weekday::Fri, 4);
//! assert_eq!(pastover.in_year(2021), NaiveDate::from_ymd(2021, 4, 2));
//! assert_eq!(pastover, NaiveDate::from_ymd(2021, 4, 2));
//! assert_eq!(pastover, NaiveDate::from_ymd(2022, 4, 1));
//! ```

use chrono::prelude::*;

mod eq;
pub mod before_after;
pub mod holidays;
pub mod iter;

pub use before_after::*;
pub use iter::*;
use HolidayDate::*;

/// An annually repeating calendar date.
/// Can be either a fixed date (e.g., April 1) or an nth weekday of the month (e.g., 4th Thursday
/// in November)
#[derive(Debug, Clone, Copy)]
pub struct Holiday<S> {
    name: S,
    date: HolidayDate,
}

impl<S: ToString> Holiday<S> {
    /// Creates a new fixed date holiday
    pub fn new_fixed(name: S, month: u32, day: u32) -> Self {
        Holiday {
            name,
            date: HolidayDate::FixedDate(DayOfMonth { month, day }),
        }
    }

    /// Creates a new nth weekday of the month Holiday
    pub fn new_nth(name: S, nth: u32, weekday: Weekday, month: u32) -> Self {
        Holiday {
            name,
            date: HolidayDate::NthDate(NthWeekdayOfMonth::new(nth, weekday, month)),
        }
    }

    /// Returns a reference to the Name of the Holiday
    pub fn name(&self) -> &S {
        &self.name
    }

    /// Returns an iterator over all the occurrences of a given Holiday starting at the earliest
    /// representable date.
    pub fn iter(&self) -> HolidayIter<Self> {
        self.into_iter()
    }

    /// Determine the date of a Holiday in a given year
    pub fn in_year(&self, year: i32) -> NaiveDate {
        self.after(&NaiveDate::from_ymd(year, 1, 1))
    }
}

#[test]
fn holiday_in_year() {
    assert_eq!(holidays::global::CHRISTMAS.in_year(2020), NaiveDate::from_ymd(2020, 12, 25));
    assert_eq!(holidays::united_states::THANKSGIVING.in_year(2020), NaiveDate::from_ymd(2020, 11, 26));
    assert_eq!(holidays::global::NEW_YEARS_DAY.in_year(2020), NaiveDate::from_ymd(2020, 1, 1));
    assert_eq!(holidays::global::NEW_YEARS_EVE.in_year(2020), NaiveDate::from_ymd(2020, 12, 31));
}

/// Holiday Date type
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum HolidayDate {
    /// Fixed date. Example: "October 31"
    FixedDate(DayOfMonth),

    /// Relative weekday in a month. Example: "4th Thursday in November"
    NthDate(NthWeekdayOfMonth),
}

impl HolidayDate {
    /// Returns an iterator over the ocurrences of the HolidayDate
    pub fn iter(&self) -> HolidayIter<Self> {
        self.into_iter()
    }
}

/// A fixed day of the month (e.g.:  March 31)
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct DayOfMonth {
    /// The day of the month
    pub day: u32,
    /// The month (January = 1)
    pub month: u32,
}

impl DayOfMonth {
    /// Create a new DayOfMonth
    pub fn new(day: u32, month: u32) -> Self {
        DayOfMonth { day, month }
    }

    /// Returns an iterator over the ocurrences of the DayOfMonth
    pub fn iter(&self) -> HolidayIter<Self> {
        self.into_iter()
    }
}

/// Nth weekday of a month (e.g.: Second Tuesday in October)
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct NthWeekdayOfMonth {
    nth: NthWeekday,
    weekday: Weekday,
    month: u32,
}

impl NthWeekdayOfMonth {
    /// Creates a new NthWeekdayOfMonth
    pub fn new<N: Into<NthWeekday>>(nth: N, weekday: Weekday, month: u32) -> Self {
        NthWeekdayOfMonth {
            nth: nth.into(),
            weekday,
            month,
        }
    }

    /// Returns an iterator over the ocurrences of the NthWeekdayOfMonth
    pub fn iter(&self) -> HolidayIter<Self> {
        self.into_iter()
    }
}

impl From<NaiveDate> for NthWeekdayOfMonth {
    fn from(date: NaiveDate) -> Self {
        let mut nth = 0;
        let mut loop_date = date.clone().with_day(1).expect("invalid day of month");

        loop {
            if loop_date.weekday() == date.weekday() {
                nth += 1;
            }

            if &loop_date >= &date {
                break;
            }

            loop_date = loop_date.succ();
        }

        NthWeekdayOfMonth {
            nth: nth.into(),
            weekday: date.weekday(),
            month: date.month(),
        }
    }
}

/// The nth ocurrence of a weekday in a month.
///
/// Using the `Fifth` explicitly may panic if you try
/// to create a date with it, as some months do not have 5 ocurrences of a given weekday.
#[allow(missing_docs)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub enum NthWeekday {
    First  = 1,
    Second = 2,
    Third  = 3,
    Fourth = 4,
    Fifth  = 5,
    Last   = 6,
}

impl From<u32> for NthWeekday {
    fn from(u: u32) -> NthWeekday {
        use NthWeekday::*;
        match u {
            0 => panic!("value must be non-zero"),
            1 => First,
            2 => Second,
            3 => Third,
            4 => Fourth,
            5 => Fifth,
            _ => Last,
        }
    }
}

#[test]
fn nth_weekday() {
    dbg!(NthWeekday::Last as u32);
}

#[test]
fn tgives() {
    let tgives = NthWeekdayOfMonth::new(4, Weekday::Thu, 11);

    dbg!(tgives.after_today());
    dbg!(tgives.before_today());

    dbg!(NthWeekdayOfMonth::from(NaiveDate::from_ymd(2020, 6, 8)));
}
