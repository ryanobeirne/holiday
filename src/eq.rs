use crate::*;

use std::cmp::Ordering;

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

impl<S: ToString> PartialEq for Holiday<S> {
    fn eq(&self, other: &Self) -> bool {
        self.date == other.date && self.name.to_string() == other.name.to_string()
    }
}

impl<S: ToString> Eq for Holiday<S> {}

impl<S: ToString> Ord for Holiday<S> {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.date == other.date {
            self.name.to_string().cmp(&other.name.to_string())
        } else {
            self.date.cmp(&other.date)
        }
    }
}

impl<S: ToString> PartialOrd for Holiday<S> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialOrd for HolidayDate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HolidayDate {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (FixedDate(self_dom), FixedDate(other_dom)) => self_dom.cmp(other_dom),
            (NthDate(self_nwom), NthDate(other_nwom)) => self_nwom.cmp(other_nwom),
            (FixedDate(self_dom), NthDate(other_nwom)) => {
                if self_dom.month == other_nwom.month {
                    Ordering::Less
                } else {
                    self_dom.month.cmp(&other_nwom.month)
                }
            }
            (NthDate(self_nwom), FixedDate(other_dom)) => {
                if self_nwom.month == other_dom.month {
                    Ordering::Greater
                } else {
                    self_nwom.month.cmp(&other_dom.month)
                }
            }
        }
    }
}

impl PartialOrd for DayOfMonth {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DayOfMonth {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.month == other.month {
            self.day.cmp(&other.day)
        } else {
            self.month.cmp(&other.month)
        }
    }
}

impl PartialEq<NaiveDate> for DayOfMonth {
    fn eq(&self, date: &NaiveDate) -> bool {
        self.month == date.month() && self.day == date.day()
    }
}


impl PartialOrd for NthWeekdayOfMonth {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.month == other.month {
            if self.nth == other.nth {
                self.weekday
                    .num_days_from_sunday()
                    .partial_cmp(&other.weekday.num_days_from_sunday())
            } else {
                self.nth.partial_cmp(&other.nth)
            }
        } else {
            self.month.partial_cmp(&other.month)
        }
    }
}

impl Ord for NthWeekdayOfMonth {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.month == other.month {
            if self.nth == other.nth {
                self.weekday
                    .num_days_from_sunday()
                    .cmp(&other.weekday.num_days_from_sunday())
            } else {
                self.nth.cmp(&other.nth)
            }
        } else {
            self.month.cmp(&other.month)
        }
    }
}

impl PartialEq<NaiveDate> for NthWeekdayOfMonth {
    fn eq(&self, date: &NaiveDate) -> bool {
        if self.nth == NthWeekday::Last && date.weekday() == self.weekday {
            date.is_last_weekday()
        } else {
            self == &NthWeekdayOfMonth::from(*date)
        }
    }
}

impl PartialEq<u32> for NthWeekday {
    fn eq(&self, u: &u32) -> bool {
        use NthWeekday::*;
        match (self, u) {
            (First, 1) | (Second, 2) | (Third, 3) | (Fourth, 4) | (Fifth, 5) => true,
            _ => false,
        }
    }
}

