# holiday

A rust library for defining and iterating over annually repeating dates and holidays.

## Create a new Holiday

A `Holiday` can be either a fixed date like 'April 2nd' or an nth weekday of a month, like '1st Friday in April'.

```rust
use holiday::*;
use chrono::{Weekday, NaiveDate};

// Regular `fixed` holiday
let holiday = Holiday::new_fixed("April 2nd", April, 2);
assert_eq!(holiday.in_year(2021), NaiveDate::from_ymd(2021, 4, 2));
assert_eq!(holiday, NaiveDate::from_ymd(2021, 4, 2));
assert_eq!(holiday, NaiveDate::from_ymd(2022, 4, 2));

// Pastover: First Friday in April, an `nth` holiday
let pastover = Holiday::new_nth("Pastover", First, Weekday::Fri, April);
assert_eq!(pastover.in_year(2021), NaiveDate::from_ymd(2021, 4, 2));
assert_eq!(pastover, NaiveDate::from_ymd(2021, 4, 2));
assert_eq!(pastover, NaiveDate::from_ymd(2022, 4, 1));
```

## Iterate over the occurrences of a Holiday.

The `HolidayIter` type is an iterator over the occurrences of a Holiday.
