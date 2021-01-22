use holiday::*;
use two_timer::parse;

const USAGE: &str = r#"daysto: Command line date counter. How many days until...?

USAGE:
    daysto <date>

EXAMPLES:
    daysto '2020-3-31'
    daysto '31 March 2021'
    daysto 'March 31, 2021'
    daysto 'November 2'
    daysto 'This Friday'
    daysto Christmas
    daysto Thanksgiving
    daysto '1 day after tomorrow'
    daysto '30 days ago'"#;

const FORMAT: &str = "%a %e %b %Y";

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();

    if args.is_empty() {
        eprintln!("{}", USAGE);
        std::process::exit(1);
    } else if args.contains(&"--help".into()) || args.contains(&"-h".into()) {
        println!("{}", USAGE);
        std::process::exit(0);
    }

    for arg in args {
        match arg.parse::<Holiday<_>>() {
            Ok(holiday) => {
                let next = holiday.after_today();
                let days = days_to(next);
                println!("Days until {} ({}): {}", holiday.name(), next.format(FORMAT), days);
            },
            Err(_) => {
                match parse(arg.as_str(), None) {
                    Ok((first, _second, _is_range)) => {
                        let days = days_to(first.date());
                        println!("Days until {}: {}", first.format(FORMAT), days);
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

