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
pub struct Holiday<S: ToString> {
    name: S,
    date: HolidayDate,
}

impl<S: ToString> Holiday<S> {
    pub fn new_fixed(name: S, month: u32, day: u32) -> Self {
        Holiday {
            name,
            date: HolidayDate::FixedDate(DayOfMonth { month, day }),
        }
    }

    pub fn new_nth(name: S, nth: u32, weekday: Weekday, month: u32) -> Self {
        Holiday {
            name,
            date: HolidayDate::NthDate(NthWeekdayOfMonth::new(nth, weekday, month)),
        }
    }

    pub fn name(&self) -> &S {
        &self.name
    }

    pub fn iter(&self) -> HolidayIter<Self> {
        self.into_iter()
    }
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
    pub fn iter(&self) -> HolidayIter<Self> {
        self.into_iter()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct DayOfMonth {
    pub day: u32,
    pub month: u32,
}

impl DayOfMonth {
    pub fn new(day: u32, month: u32) -> Self {
        DayOfMonth { day, month }
    }

    pub fn iter(&self) -> HolidayIter<Self> {
        self.into_iter()
    }
}

/// nth weekday of a month
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct NthWeekdayOfMonth {
    nth: NthWeekday,
    weekday: Weekday,
    month: u32,
}

impl NthWeekdayOfMonth {
    pub fn new<N: Into<NthWeekday>>(nth: N, weekday: Weekday, month: u32) -> Self {
        NthWeekdayOfMonth {
            nth: nth.into(),
            weekday,
            month,
        }
    }

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
