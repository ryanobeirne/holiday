//! Holidays in the United States

use crate::*;
use NthWeekday::*;

holiday_const!(
    /// Martin Luther King Jr. Day: 3rd Monday in January
    MLKJ_DAY, "Martin Luther King Jr. Day", Third, Weekday::Mon, January
);
holiday_const!(
    /// Groundhog Day: February 2
    GROUNDHOG_DAY, "Groundhog Day", February, 2
);
holiday_const!(
    /// Super Bowl Sunday: 1st Sunday in February
    SUPERBOWL_SUNDAY, "Super Bowl Sunday", First, Weekday::Sun, February
);
holiday_const!(
    /// President's Day: 3rd Monday in February
    PRESIDENTS_DAY, "President's Day", Third, Weekday::Mon, February
);
holiday_const!(
    /// Valentine's Day: February 14
    VALENTINES_DAY, "Valentine's Day", February, 14
);
holiday_const!(
    /// Daylight Saving Time Starts: 2nd Sunday in March
    DST_START, "Daylight Saving Time Starts", Second, Weekday::Sun, March
);
holiday_const!(
    /// April Fool's Day: April 1
    APRIL_FOOLS_DAY, "April Fool's Day", April, 1
);
holiday_const!(
    /// Kentucky Derby: 1st Saturday in May
    KENTUCKY_DERBY, "Kentucky Derby", First, Weekday::Sat, May
);
holiday_const!(
    /// Memorial Day: Last Monday in May
    MEMORIAL_DAY, "Memorial Day", Last, Weekday::Mon, May
);
holiday_const!(
    /// Mother's Day: 2nd Sunday in May
    MOTHERS_DAY, "Mother's Day", Second, Weekday::Sun, May
);
holiday_const!(
    /// Flag Day: June 14
    FLAG_DAY, "Flag Day", June, 14
);
holiday_const!(
    /// Independence Day: July 4
    INDEPENDENCE_DAY, "Independence Day", July, 4
);
holiday_const!(
    /// Father's Day: 3rd Sunday in June
    FATHERS_DAY, "Father's Day", Third, Weekday::Sun, June
);
holiday_const!(
    /// Labor Day: 1st Monday in September
    LABOR_DAY, "Labor Day", First, Weekday::Mon, September
);
holiday_const!(
    /// Halloween: October 31
    HALLOWEEN, "Halloween", October, 31
);
holiday_const!(
    /// Columbus Day: 2nd Monday in October
    COLUMBUS_DAY, "Columbus Day", Second, Weekday::Mon, October
);
holiday_const!(
    /// Veteran's Day: November 11
    VETERANS_DAY, "Veteran's Day", November, 11
);
holiday_const!(
    /// Daylight Saving Time Ends: 1st Sunday in November
    DST_END, "Daylight Saving Time Ends", First, Weekday::Sun, November
);
holiday_const!(
    /// Thanksgiving: 4th Thursday in November
    THANKSGIVING, "Thanksgiving", Fourth, Weekday::Thu, November
);

#[test]
fn holiday_eq() {
    assert_eq!(THANKSGIVING, NthWeekdayOfMonth::new(4, Weekday::Thu, 11));
    assert_eq!(THANKSGIVING, NaiveDate::from_ymd(2020, 11, 26));
    assert_eq!(THANKSGIVING, NaiveDate::from_ymd(2021, 11, 25));
    assert_eq!(HALLOWEEN, NaiveDate::from_ymd(2020, 10, 31));
    assert_eq!(HALLOWEEN, NaiveDate::from_ymd(2021, 10, 31));
}

#[test]
fn holiday_ord() {
    let mut vec = vec![
        super::global::NEW_YEARS_EVE,
        THANKSGIVING,
        super::global::NEW_YEARS_DAY,
        HALLOWEEN,
        super::global::CHRISTMAS,
    ];

    vec.sort();

    dbg!(&vec);

    let exp = vec![
        super::global::NEW_YEARS_DAY,
        HALLOWEEN,
        THANKSGIVING,
        super::global::CHRISTMAS,
        super::global::NEW_YEARS_EVE,
    ];

    assert_eq!(vec, exp);
}
