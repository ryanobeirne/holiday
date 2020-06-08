use crate::*;

#[derive(Debug)]
pub struct HolidayIter<'h, H: BeforeAfterDate> {
    holiday: &'h H,
    first: NaiveDate,
    last: NaiveDate,
    current: NaiveDate,
}

impl<'h, H: BeforeAfterDate> HolidayIter<'h, H> {
    /// Set the current date
    pub fn at(mut self, current_date: NaiveDate) -> Self {
        self.current = current_date.pred();
        self.shift_from(current_date);

        self
    }

    /// Start the iterator at the given date
    pub fn starting_at(mut self, start_date: NaiveDate) -> Self {
        self.first = self.holiday.after(&start_date);
        self.shift_from(start_date);

        self
    }

    /// End the iterator at the given date
    pub fn ending_at(mut self, end_date: NaiveDate) -> Self {
        self.last = self.holiday.before(&end_date);
        self.shift_from(end_date);

        self
    }

    /// Set first and last dates for the iterator if a new date is not between the first
    /// and last dates.
    pub fn shift_from(&mut self, date: NaiveDate) {
        if date < self.first {
            self.first = date;
        }

        if date > self.last {
            self.last = date;
        }
    }
}

impl<'h, H: BeforeAfterDate> Iterator for HolidayIter<'h, H> {
    type Item = NaiveDate;
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.holiday.after(&self.current.succ());
        if next <= self.last {
            self.current = next;
            Some(next)
        } else {
            None
        }
    }
}
            
impl<'h, H: BeforeAfterDate> DoubleEndedIterator for HolidayIter<'h, H> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let prev = self.holiday.before(&self.current);
        if self.current >= self.first {
            self.current = prev;
            Some(prev)
        } else {
            None
        }
    }
}

impl<'h, S: ToString> IntoIterator for &'h Holiday<S> {
    type Item = NaiveDate;
    type IntoIter = HolidayIter<'h, Holiday<S>>;
    fn into_iter(self) -> Self::IntoIter {
        HolidayIter {
            holiday: &self,
            first: self.first_date(),
            last: self.last_date(),
            current: self.first_date(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use holidays::united_states::*;

    #[test]
    fn tgives_iter() {
        dbg!(THANKSGIVING.first_date());
        dbg!(THANKSGIVING.last_date());

        let mut tgives = THANKSGIVING.into_iter().at(NaiveDate::from_ymd(2020, 11, 1));
        assert_eq!(tgives.next(), Some(NaiveDate::from_ymd(2020, 11, 26)));
        assert_eq!(tgives.next(), Some(NaiveDate::from_ymd(2021, 11, 25)));
        assert_eq!(tgives.next(), Some(NaiveDate::from_ymd(2022, 11, 24)));
        assert_eq!(tgives.next_back(), Some(NaiveDate::from_ymd(2021, 11, 25)));
        assert_eq!(tgives.next_back(), Some(NaiveDate::from_ymd(2020, 11, 26)));
        assert_eq!(tgives.next_back(), Some(NaiveDate::from_ymd(2019, 11, 28)));
    }

    #[test]
    fn tgives_count() {
        let mut dates = std::collections::HashMap::new();

        for date in THANKSGIVING.into_iter()
            .at(NaiveDate::from_ymd(2020, 11, 1))
            .ending_at(NaiveDate::from_ymd(12020, 11, 30))
        {
            //dbg!(&date);
            *dates.entry(date.day()).or_insert(0) += 1;
        }

        let sum = dates.values().sum::<usize>();
        for (day, count) in dates.iter() {
            let pct = *count as f64 / sum as f64 * 100.0;
            println!("DAY: {}: {:.2}%", day, pct);
        }
        
        let most_days = dates.into_iter()
            .max_by(|(_k0,v0),(_k1,v1)| v0.cmp(v1))
            .unwrap()
            .0;

        assert_eq!(most_days, 26);
    }

    #[test]
    fn jan_dec() {
        use NthWeekday::*;
        let jan = holiday!("First Wednesday in January", First, Weekday::Wed, 1);
        let mut jan_iter = jan.into_iter().at(NaiveDate::from_ymd(2020, 1, 1));
        assert_eq!(jan_iter.next(), Some(NaiveDate::from_ymd(2020, 1, 1)));
        assert_eq!(jan_iter.next(), Some(NaiveDate::from_ymd(2021, 1, 6)));
        assert_eq!(jan_iter.next(), Some(NaiveDate::from_ymd(2022, 1, 5)));
        assert_eq!(jan_iter.next(), Some(NaiveDate::from_ymd(2023, 1, 4)));

        let dec = holiday!("Fifth Wednesday in December", Fifth, Weekday::Wed, 12);
        let mut dec_iter = dec.into_iter().at(NaiveDate::from_ymd(2020, 12, 1));
        assert_eq!(dec_iter.next(), Some(NaiveDate::from_ymd(2020, 12, 30)));
        assert_eq!(dec_iter.next(), Some(NaiveDate::from_ymd(2021, 12, 29)));
        assert_eq!(dec_iter.next(), Some(NaiveDate::from_ymd(2025, 12, 31)));
    }
}
