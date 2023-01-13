
//! # Calendar Calculator
//!
//! The calendar_calculator crate is a simple library crate that provides a range of functionalities for calculating distances
//! between two dates expressed in str slice format, as well as adding different durations to a given date. With this crate,
//! users can easily and quickly perform calculations related to dates and distances.


use std::{fmt, io::{ErrorKind, self}};
use chrono::{Duration, Weekday, Datelike, NaiveDateTime, Timelike, Months, Local};
use regex::Regex;

/// Express the distance between two given datetimes in a lot of different formats
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

impl fmt::Display for Distance{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(
            f,
            "
            {{
                seconds: {},
                minutes: {},
                hours: {},
                days: {},
                weeks: {},
                months: {},
                years: {},
                sundays: {},
                saturdays: {},
                working_days: {},
            }}
            ",
            self.seconds,
            self.minutes,
            self.hours,
            self.days,
            self.weeks,
            self.months,
            self.years,
            self.sundays,
            self.saturdays,
            self.working_days
        )
    }
}

fn num_sundays(date1: NaiveDateTime, date2: NaiveDateTime) -> i64 {
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

fn num_saturdays(date1: NaiveDateTime, date2: NaiveDateTime) -> i64 {
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

pub trait CalendarCalculator {
    /// check if a given &str date has Y-m-d H:M:S standard format
    fn to_date(self) -> Result<NaiveDateTime, io::Error>;
    /// add n seconds to given &str datetime
    fn add_seconds(self, seconds: i64) -> (i64, i64, i64, i64, i64, i64);
    /// add n minutes to given &str datetime
    fn add_minutes(self, minutes: i64) -> (i64, i64, i64, i64, i64, i64);
    /// add n hours to given &str datetime
    fn add_hours(self, hours: i64) -> (i64, i64, i64, i64, i64, i64);
    /// add n days to given &str datetime
    fn add_days(self, days: i64) -> (i64, i64, i64, i64, i64, i64);
    /// add n weeks to given &str datetime
    fn add_weeks(self, weeks: i64) -> (i64, i64, i64, i64, i64, i64);
    /// add n months to given &str datetime
    fn add_months(self, months: i64) -> (i64, i64, i64, i64, i64, i64);
    /// add n years to given &str datetime
    fn add_years(self, years: i64) -> (i64, i64, i64, i64, i64, i64);
    /// Calculate the distance between two given &str date, return a [`Distance`]
    fn distance_between(&self, second: &str) -> Distance;
}

impl CalendarCalculator for &str{
   
    /// check if a given &str date has Y-m-d H:M:S standard format
    fn to_date(self) -> Result<NaiveDateTime, io::Error> { 
        let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2}):(\d{2})").unwrap();

        if !re.is_match(self) {
            return Err(io::Error::new(ErrorKind::Other, "Format Must Be Y-m-d H:M:S"));
        }
        Ok(NaiveDateTime::parse_from_str(self, "%Y-%m-%d %H:%M:%S").unwrap())
    }

    /// add n seconds to given &str datetime
    fn add_seconds(self, seconds: i64) -> (i64, i64, i64, i64, i64, i64) { 
        return_format(self.to_date().unwrap() + Duration::seconds(seconds))
    }

    /// add n minutes to given &str datetime
    fn add_minutes(self, minutes: i64) -> (i64, i64, i64, i64, i64, i64) { 
        return_format(self.to_date().unwrap() + Duration::minutes(minutes))
    }

    /// add n hours to given &str datetime
    fn add_hours(self, hours: i64) -> (i64, i64, i64, i64, i64, i64) { 
        return_format(self.to_date().unwrap() + Duration::hours(hours))
    }

    /// add n days to given &str datetime
    fn add_days(self, days: i64) -> (i64, i64, i64, i64, i64, i64) { 
        return_format(self.to_date().unwrap() + Duration::days(days))
    }

    /// add n months to given &str datetime
    fn add_months(self, months: i64) -> (i64, i64, i64, i64, i64, i64) {
        return_format(self.to_date().unwrap().checked_add_months(Months::new(months as u32)).unwrap())
    }

    /// add n weeks to given &str datetime
    fn add_weeks(self, weeks: i64) -> (i64, i64, i64, i64, i64, i64) { 
        return_format(self.to_date().unwrap() + Duration::days(weeks*7))
    }

    /// add n years to given &str datetime
    fn add_years(self, years: i64) -> (i64, i64, i64, i64, i64, i64) { 
        let year = self.to_date().unwrap().year() as i64 + years;
        return_format(self.to_date().unwrap().with_year(year as i32).unwrap())
    }
    
    /// Calculate the distance between two given &str date, return a [`Distance`]
    fn distance_between(&self, second: &str) -> Distance{
        let mut date1 = self.to_date().unwrap();
        let mut date2 = second.to_date().unwrap();
    
        if date2 < date1 {
            let wrap = date2;
            date2 = date1;
            date1 = wrap;
        }
    
        Distance::new((date2-date1).num_seconds(), num_sundays(date1, date2), num_saturdays(date1, date2))
    }

}

fn return_format(sum: NaiveDateTime) -> (i64, i64, i64, i64, i64, i64){
    (
        (sum).year() as i64,
        (sum).month() as i64,
        (sum).day() as i64,
        (sum).hour() as i64,
        (sum).minute() as i64,
        (sum).second() as i64,
    )
}

pub fn now() -> String{
    let n = Local::now();
    let format = "%Y-%m-%d %H:%M:%S";
    n.format(format).to_string()
}

//pub fn days()

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance(){
        //assert_eq!(1, first.distanceFrom(second));
        let first = "2001-02-18 10:00:00";
        // let first = now();
        // let first = &first[..];
        let second = "1997-07-12 10:00:00";
        println!("{}", first.distance_between(second));
        assert_eq!(first.distance_between(second).days, 1317);
        assert_eq!(first.distance_between(second).sundays, 189);
        assert_eq!(first.distance_between(second).saturdays, 189);
    }

    #[test]
    fn add_distance(){
        let date = "2023-01-12 00:00:00";
        assert_eq!(date.add_seconds(86400), (2023, 01, 13, 0,0,0));
        assert_eq!(date.add_minutes(10), (2023, 01, 12, 0,10,0));
        assert_eq!(date.add_hours(1), (2023, 01, 12, 01,0,0));
        assert_eq!(date.add_days(1), (2023, 01, 13, 0,0,0));
        assert_eq!(date.add_weeks(1), (2023, 01, 19, 0,0,0));
        assert_eq!(date.add_years(1), (2024, 01, 12, 0,0,0));
    }
    
}
