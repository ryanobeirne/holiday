#![warn(missing_docs)]

//! # holiday
//!
//! A library for defining annually repeating dates and holidays
//!
//! ## Create a new Holiday
//!
//! ```rust
//! use holiday::*;
//! use chrono::{Weekday, NaiveDate};
//!
//! // Pastover: First Friday in April, an `nth` holiday
//! let pastover = Holiday::new_nth("Pastover", First, Weekday::Fri, April);
//! assert_eq!(pastover.in_year(2021), NaiveDate::from_ymd(2021, 4, 2));
//! assert_eq!(pastover, NaiveDate::from_ymd(2021, 4, 2));
//! assert_eq!(pastover, NaiveDate::from_ymd(2022, 4, 1));
//!
//! // Regular `fixed` holiday
//! let holiday = Holiday::new_fixed("April 2nd", April, 2);
//! assert_eq!(holiday.in_year(2021), NaiveDate::from_ymd(2021, 4, 2));
//! assert_eq!(holiday, NaiveDate::from_ymd(2021, 4, 2));
//! assert_eq!(holiday, NaiveDate::from_ymd(2022, 4, 2));
//! ```

use chrono::prelude::*;

mod eq;
pub mod before_after;
pub mod holidays;
pub mod iter;

pub use before_after::*;
pub use iter::*;
use HolidayDate::*;
pub use NthWeekday::*;
pub use Month::*;

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
    pub fn new_fixed<M: Into<Month>>(name: S, month: M, day: u32) -> Self {
        Holiday {
            name,
            date: HolidayDate::FixedDate(DayOfMonth { month: month.into(), day }),
        }
    }

    /// Creates a new nth weekday of the month Holiday
    pub fn new_nth<N: Into<NthWeekday>, M: Into<Month>>(name: S, nth: N, weekday: Weekday, month: M) -> Self {
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
    pub month: Month,
}

impl DayOfMonth {
    /// Create a new DayOfMonth
    pub fn new<M: Into<Month>>(day: u32, month: M) -> Self {
        DayOfMonth { day, month: month.into() }
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
    month: Month,
}

impl NthWeekdayOfMonth {
    /// Creates a new NthWeekdayOfMonth
    pub fn new<N: Into<NthWeekday>, M: Into<Month>>(nth: N, weekday: Weekday, month: M) -> Self {
        NthWeekdayOfMonth {
            nth: nth.into(),
            weekday,
            month: month.into(),
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
            month: date.month().into(),
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

impl From<NthWeekday> for u32 {
    fn from(nth: NthWeekday) -> Self {
        nth as u32
    }
}

/// A convenience enum for specifiying the month (January = 1)
#[allow(missing_docs)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub enum Month {
    January   = 1,
    February  = 2,
    March     = 3,
    April     = 4,
    May       = 5,
    June      = 6,
    July      = 7,
    August    = 8,
    September = 9,
    October   = 10,
    November  = 11,
    December  = 12,
}

impl Month {
    /// Get the month as a `u32` where January = 0
    pub fn from_zero(&self) -> u32 {
        *self as u32 - 1
    }
}

impl From<u32> for Month {
    fn from(u: u32) -> Self {
        match u {
             1  => January,
             2  => February,
             3  => March,
             4  => April,
             5  => May,
             6  => June,
             7  => July,
             8  => August,
             9  => September,
             10 => October,
             11 => November,
             12 => December,
             u  => panic!("Invalid month: '{}'", u),
        }
    }
}

impl From<Month> for u32 {
    fn from(m: Month) -> Self {
        m as u32
    }
}

#[test]
fn tgives_nth_weekday_of_month() {
    let tgives = NthWeekdayOfMonth::new(Fourth, Weekday::Thu, 11);

    dbg!(tgives.after_today());
    dbg!(tgives.before_today());

    dbg!(NthWeekdayOfMonth::from(NaiveDate::from_ymd(2020, 6, 8)));
}
