

use chrono::{ Utc, NaiveDate, Duration, Weekday, Datelike};

pub fn printnow() -> chrono::DateTime<Utc>{
    let now = Utc::now();
    now
}

pub struct Distance {
    seconds: i64,
    minutes: i64,
    hours: i64,
    days: i64,
    weeks: i64,
    months: i64,
    years: i64,
    sundays: i64,
    saturdays: i64,
    working_days: i64
}

impl Distance{
    pub fn new(seconds: i64, sundays: i64, saturdays: i64) -> Self {
        Distance{
            seconds,
            minutes: Duration::seconds(seconds).num_minutes(),
            hours: Duration::seconds(seconds).num_hours(),
            days: Duration::seconds(seconds).num_days(),
            weeks: Duration::seconds(seconds).num_weeks(),
            months: Duration::seconds(seconds).num_days() / 30,
            years: Duration::seconds(seconds).num_days() / 365,
            sundays,
            saturdays,
            working_days: Duration::seconds(seconds).num_days() - sundays,
        }
    }
}

fn num_sundays(date1: NaiveDate, date2: NaiveDate) -> i64 {
    let days = (date2-date1).num_days();
    let today = match date1.weekday() {
        Weekday::Mon => 6,
        Weekday::Tue => 5,
        Weekday::Wed => 4,
        Weekday::Thu => 3,
        Weekday::Fri => 2,
        Weekday::Sat => 1,
        Weekday::Sun => 0,
    };

    (days - today) / 7 + 1
}

fn num_saturday(date1: NaiveDate, date2: NaiveDate) -> i64 {
    let days = (date2-date1).num_days();
    let today = match date1.weekday() {
        Weekday::Mon => 5,
        Weekday::Tue => 4,
        Weekday::Wed => 3,
        Weekday::Thu => 2,
        Weekday::Fri => 1,
        Weekday::Sat => 0,
        Weekday::Sun => 6,
    };

    (days - today) / 7 + 1
}

trait StrToDate {
    fn to_date(self, fmt: &str) -> NaiveDate;
}

impl StrToDate for &str{
    fn to_date(self, fmt: &str) -> NaiveDate {
        NaiveDate::parse_from_str(date, fmt).expect("Date wrong")
    }
}


pub fn distance_between(first: &str, second: &str) -> Distance{
    let date1 = first.to_date("%Y-%m-%d").unwrap();
    let date2 = second.to_date("%Y-%m-%d").unwrap();

    if date2 < date1 {
        let wrap = date2;
        date2 = date1;
        date1 = wrap;
    } else {
        date1 - date2
    };

    Distance::new((range).num_seconds(), num_sundays(date1, date2), num_saturday(date1, date2))
}

pub fn days()


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_date(){
       println!("{}", printnow());
    }

    #[test]
    fn distance(){
        //assert_eq!(1, first.distanceFrom(second));
        let first = "2023-01-12";
        let second = "2024-05-08";
        assert_eq!(distance_between(first, second).days, 482);
        assert_eq!(distance_between(first, second).sundays, 69);
        assert_eq!(distance_between(first, second).saturdays, 69);
    }

    fn add Distance(){

    }
    
}
