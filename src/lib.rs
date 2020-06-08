use chrono::prelude::*;

pub mod before_after;
pub mod holidays;
pub mod iter;

pub use before_after::*;
pub use iter::*;

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
}

impl<S: ToString> PartialEq<NaiveDate> for Holiday<S> {
    fn eq(&self, date: &NaiveDate) -> bool {
        match &self.date {
            HolidayDate::FixedDate(fixed) => fixed == date,
            HolidayDate::NthDate(nth) => nth == date,
        }
    }
}

impl<S: ToString> PartialEq<NthWeekdayOfMonth> for Holiday<S> {
    fn eq(&self, nth: &NthWeekdayOfMonth) -> bool {
        if let HolidayDate::NthDate(self_nth) = self.date {
            &self_nth == nth
        } else {
            false
        }
    }
}

/// Holiday Date type
#[derive(Debug, Clone, Copy)]
pub enum HolidayDate {
    /// Fixed date. Example: "October 31"
    FixedDate(DayOfMonth),

    /// Relative weekday in a month. Example: "4th Thursday in November"
    NthDate(NthWeekdayOfMonth),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct DayOfMonth {
    month: u32,
    day: u32,
}

impl PartialEq<NaiveDate> for DayOfMonth {
    fn eq(&self, date: &NaiveDate) -> bool {
        self.month == date.month() && self.day == date.day()
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

impl PartialEq<NaiveDate> for NthWeekdayOfMonth {
    fn eq(&self, date: &NaiveDate) -> bool {
        self == &NthWeekdayOfMonth::from(*date)
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

impl PartialEq<u32> for NthWeekday {
    fn eq(&self, u: &u32) -> bool {
        use NthWeekday::*;
        match (self, u) {
            (First, 1) | (Second, 2) | (Third, 3) | (Fourth, 4) | (Fifth, 5) => true,
            _ => false
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

