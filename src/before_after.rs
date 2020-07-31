//! # BeforeAfterDate
//!
//! The `BeforeAfterDate` trait 

use crate::*;

/// Trait to determine the next and previous occurrence of dates (successor and predecessor).
/// The next occurrence should include the current date, the previous occurrence should exclude the current date.
pub trait BeforeAfterDate {
    /// The next occurrence after a given date
    fn after(&self, date: &NaiveDate) -> NaiveDate;

    /// The previous occurrence before a given date
    fn before(&self, date: &NaiveDate) -> NaiveDate;

    /// The next occurrence including today (successor)
    fn after_today(&self) -> NaiveDate {
        self.after(&Local::today().naive_local())
    }

    /// The previous occurrence excluding today (predecessor)
    fn before_today(&self) -> NaiveDate {
        self.before(&Local::today().naive_local())
    }

    /// The first representable occurrence of the date
    fn first_date(&self) -> NaiveDate {
        self.after(&chrono::MIN_DATE.naive_local())
    }

    /// The last representable occurrence of the date
    fn last_date(&self) -> NaiveDate {
        self.before(&chrono::MAX_DATE.naive_local())
    }
}

impl<S: ToString> BeforeAfterDate for Holiday<S> {
    fn after(&self, date: &NaiveDate) -> NaiveDate {
        self.date.after(date)
    }

    fn before(&self, date: &NaiveDate) -> NaiveDate {
        self.date.before(date)
    }
}

impl BeforeAfterDate for HolidayDate {
    fn after(&self, date: &NaiveDate) -> NaiveDate {
        match self {
            HolidayDate::FixedDate(day_of_month) => day_of_month.after(date),
            HolidayDate::NthDate(nth) => nth.after(date),
        }
    }

    fn before(&self, date: &NaiveDate) -> NaiveDate {
        match self {
            HolidayDate::FixedDate(day_of_month) => day_of_month.before(date),
            HolidayDate::NthDate(nth) => nth.before(date),
        }
    }
}

impl BeforeAfterDate for DayOfMonth {
    fn after(&self, date: &NaiveDate) -> NaiveDate {
        let mut check_date = date.clone();
        loop {
            if self == &check_date {
                break check_date;
            } else {
                check_date = check_date.succ();
            }
        }
    }

    fn before(&self, date: &NaiveDate) -> NaiveDate {
        let mut check_date = date.clone().pred();
        loop {
            if self == &check_date {
                break check_date;
            } else {
                check_date = check_date.pred();
            }
        }
    }
}

impl BeforeAfterDate for NthWeekdayOfMonth {
    fn after(&self, date: &NaiveDate) -> NaiveDate {
        let mut check_date = date.clone();
        loop {
            if self == &check_date {
                break check_date;
            } else {
                if check_date.month() < self.month {
                    check_date = check_date
                        .with_day(1)
                        .expect("invalid day: after::lt")
                        .with_month(self.month as u32)
                        .expect("invalid month: after::lt");
                } else if check_date.month() > self.month {
                    check_date = check_date
                        .with_day(1)
                        .expect("invalid day")
                        .with_month(self.month as u32)
                        .expect("invalid month: after::gt")
                        .with_year(check_date.year() + 1)
                        .expect("invalid year: after::gt");
                } else {
                    check_date = check_date.succ();
                }
            }
        }
    }

    fn before(&self, date: &NaiveDate) -> NaiveDate {
        let mut check_date = date.clone().pred();
        loop {
            if self == &check_date {
                break check_date;
            } else {
                if check_date.month() > self.month {
                    check_date = check_date
                        .with_month(self.month as u32)
                        .expect("invalid month: before::gt")
                        .last_day_of_month();
                } else if check_date.month() < self.month {
                    check_date = check_date
                        .with_day(1)
                        .expect("invalid day: before::lt")
                        .with_month(self.month as u32)
                        .expect("invalid month: before::lt")
                        .with_year(check_date.year() - 1)
                        .expect("invalid year: before::lt")
                        .last_day_of_month();
                } else {
                    check_date = check_date.pred();
                }
            }
        }
    }
}

/// Determine the last day in a given date's month
pub trait LastDayOfMonth: Datelike {
    /// Finds the last date in a given calendar month
    fn last_day_of_month(&self) -> NaiveDate;
}

impl<D: Datelike> LastDayOfMonth for D {
    fn last_day_of_month(&self) -> NaiveDate {
        let (next_month, next_year) = match self.month() {
            12 => (1, self.year() + 1),
            _ => (self.month() + 1, self.year()),
        };

        NaiveDate::from_ymd(next_year, next_month, 1).pred()
    }
}

/// Determine the first day in a given date's month
pub trait FirstDayOfMonth: Datelike {
    /// Finds the first date in a given calendar month
    fn first_day_of_month(&self) -> NaiveDate;
}

impl<D: Datelike> FirstDayOfMonth for D {
    fn first_day_of_month(&self) -> NaiveDate {
        NaiveDate::from_ymd(self.year(), self.month(), 1)
    }
}

/// Trait to determine if a NaiveDate is the last weekday of the month
pub trait IsLastWeekday: Datelike {
    /// Determine if a date is the last weekday of the month
    fn is_last_weekday(self) -> bool {
        let mut count = 0;
        let mut date = NaiveDate::from_ymd(self.year(), self.month(), self.day()).succ();
        while date <= self.last_day_of_month().succ() {
            if date.weekday() == self.weekday() {
                count += 1;
            }
            date = date.succ();
        }

        count == 0
    }
}

impl<D: Datelike> IsLastWeekday for D {}

#[test]
fn test_is_last_weekday() {
    let date = NaiveDate::from_ymd(2020, 7, 28);
    assert!(date.is_last_weekday());

    let date = NaiveDate::from_ymd(2020, 7, 21);
    assert!(!date.is_last_weekday());

    let date = NaiveDate::from_ymd(2020, 12, 31);
    assert!(date.is_last_weekday());

    let date = NaiveDate::from_ymd(2021, 1, 1);
    assert!(!date.is_last_weekday());

    let holiday = Holiday::new_nth("Last Tuesday in July", NthWeekday::Last, Weekday::Tue, 7);
    let holiday2 = Holiday::new_nth("Fourth Tuesday in July", NthWeekday::Fourth, Weekday::Tue, 7);
    let date = NaiveDate::from_ymd(2020, 7, 28);
    assert_eq!(holiday, date);
    assert_eq!(holiday2, date);
    assert_ne!(holiday, holiday2);
}

#[test]
fn test_last_day_of_month() {
    let date = NaiveDate::from_ymd(2020, 12, 31);
    assert_eq!(date.last_day_of_month(), date);

    let date = NaiveDate::from_ymd(2020, 12, 1);
    let exp = NaiveDate::from_ymd(2020, 12, 31);
    assert_eq!(date.last_day_of_month(), exp);

    let date = NaiveDate::from_ymd(2020, 1, 1);
    let exp = NaiveDate::from_ymd(2020, 1, 31);
    assert_eq!(date.last_day_of_month(), exp);

    let date = NaiveDate::from_ymd(2020, 2, 1);
    let exp = NaiveDate::from_ymd(2020, 2, 29);
    assert_eq!(date.last_day_of_month(), exp);
}

#[test]
fn test_first_day_of_month() {
    let date = NaiveDate::from_ymd(2020, 12, 1);
    assert_eq!(date.first_day_of_month(), date);

    let date = NaiveDate::from_ymd(2020, 12, 31);
    let exp = NaiveDate::from_ymd(2020, 12, 1);
    assert_eq!(date.first_day_of_month(), exp);

    let date = NaiveDate::from_ymd(2020, 1, 31);
    let exp = NaiveDate::from_ymd(2020, 1, 1);
    assert_eq!(date.first_day_of_month(), exp);

    let date = NaiveDate::from_ymd(2020, 2, 29);
    let exp = NaiveDate::from_ymd(2020, 2, 1);
    assert_eq!(date.first_day_of_month(), exp);
}
