use holiday::*;
use chrono::*;
use two_timer::*;

fn main() {
    let args = std::env::args().skip(1);
    for arg in args {
        match arg.parse::<Holiday<_>>() {
            Ok(holiday) => {
                let next = holiday.after_today();
                let days = days_to(next);
                println!("Days until {}: {}", holiday.name(), days);
            },
            Err(_) => {
                match parse(arg.as_str(), None) {
                    Ok((first, _second, _is_range)) => {
                        let days = days_to(first.date());
                        println!("Days until {}: {}", arg, days);
                    }
                    Err(_) => eprintln!("Unknown holiday: '{}'", arg),
                }
            }
        }
    }
}

fn days_to(date: NaiveDate) -> i64 {
    (date - Local::today().naive_local()).num_days()
}
