
//! # Calendar Calculator
//!
//! The calendar_calculator crate is a simple library crate that provides a range of functionalities for calculating distances
//! between two dates expressed in str slice format, as well as adding different durations to a given date. With this crate,
//! users can easily and quickly perform calculations related to dates and distances.


use std::{fmt, io::{self}, str::FromStr};
use chrono::{Duration, Weekday, Datelike, NaiveDateTime, Timelike, Months, Local, DateTime, TimeZone, NaiveDate};
use regex::Regex;

/// Express the distance between two given datetimes in a lot of different formats
pub struct Distance {
    pub seconds: i64,
    pub minutes: i64,
    pub hours: i64,
    pub days: i64,
    pub weeks: i64,
    pub months: i64,
    pub years: i64,
    pub sundays: i64,
    pub saturdays: i64,
    pub working_days: i64
}

impl Distance{
    /// Create new Instance of [`Distance`]
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

/// Display [`Distance`] as object style
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


/// Simple DateTime format
#[derive(Debug, PartialEq, Clone, Copy)]
struct StrDateTime{
    pub year: i32, 
    pub month: u32, 
    pub day: u32, 
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
}

impl StrDateTime{

    /// Create new instance of [`StrDateTime`]
    pub fn new(year: i32, month: u32, day: u32, hour: u32, minute: u32, second: u32) -> Self {
        StrDateTime{
            year,
            month,
            day,
            hour,
            minute,
            second
        }
    }

    /// check if a given [`StrDateTime`] date has Y-m-d H:M:S standard format and return [`chrono::NaiveDatetime`]
    fn to_naive(self) -> Result<NaiveDateTime, io::Error> {
        Ok(NaiveDate::from_ymd_opt(self.year as i32, self.month as u32, self.day as u32).unwrap().and_hms_opt(self.hour as u32, self.minute as u32, self.second as u32).unwrap())
    }

    /// convert [`chrono::NaiveDateTime`] to [`StrDateTime`]
    fn to_str_date_time(dt: NaiveDateTime) -> StrDateTime{
        StrDateTime::new(
            dt.year(),
            dt.month(),
            dt.day(),
            dt.hour(),
            dt.minute(),
            dt.second()
        )
    }

    /// return num of sundays between date1 and date2
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

        /// return num of saturdays between date1 and date2
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
    
        /// add n seconds to given [`StrDateTime`]
        fn add_seconds(self, seconds: i64) -> StrDateTime { 
            Self::to_str_date_time(self.to_naive().unwrap() + Duration::seconds(seconds))
        }
    
        /// add n minutes to given [`StrDateTime`]
        fn add_minutes(self, minutes: i64) -> StrDateTime { 
            Self::to_str_date_time(self.to_naive().unwrap() + Duration::minutes(minutes))
        }
    
        /// add n hours to given [`StrDateTime`]
        fn add_hours(self, hours: i64) -> StrDateTime { 
            Self::to_str_date_time(self.to_naive().unwrap() + Duration::hours(hours))
        }
    
        /// add n days to given [`StrDateTime`]
        fn add_days(self, days: i64) -> StrDateTime { 
            Self::to_str_date_time(self.to_naive().unwrap() + Duration::days(days))
        }
    
        /// add n months to given [`StrDateTime`]
        fn add_months(self, months: i32) -> StrDateTime {
            Self::to_str_date_time(self.to_naive().unwrap().checked_add_months(Months::new(months as u32)).unwrap())
        }
    
        /// add n weeks to given [`StrDateTime`]
        fn add_weeks(self, weeks: i64) -> StrDateTime { 
            Self::to_str_date_time(self.to_naive().unwrap() + Duration::days(weeks*7))
        }
    
        /// add n years to given [`StrDateTime`]
        fn add_years(self, years: i32) -> StrDateTime { 
            let year = self.to_naive().unwrap().year() + years;
            let date = self.to_naive().unwrap().with_year(year).unwrap();
            Self::to_str_date_time(date)
        }
        
        /// Calculate the distance between two given [`StrDateTime`] date, return a [`Distance`]
        fn distance_between(self, second: StrDateTime) -> Distance{
            let mut date1 = self.to_naive().unwrap();
            let mut date2 = second.to_naive().unwrap();
        
            if date2 < date1 {
                let wrap = date2;
                date2 = date1;
                date1 = wrap;
            }
        
            Distance::new((date2-date1).num_seconds(), Self::num_sundays(date1, date2), Self::num_saturdays(date1, date2))
        }

        /// Return current dateTime in [`StrDateTime`] format
        pub fn now() -> StrDateTime{
            let n = Local::now();
            let format = "%Y-%m-%d %H:%M:%S";
            let now = n.format(format).to_string().parse().unwrap();
            now
        }

}

#[derive(Debug, PartialEq, Eq)]
struct ParseStrDateTimeError;

impl FromStr for StrDateTime{
    type Err = ParseStrDateTimeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2}):(\d{2})").unwrap();

        if !re.is_match(s) {
            return Err(ParseStrDateTimeError);
        }

        let (date, time) = s.split_once(" ").unwrap();
        let mut date = date.split("-");
        let mut time = time.split(":");


        Ok(StrDateTime { 
            year: date.next().unwrap().parse().unwrap(),
            month: date.next().unwrap().parse().unwrap(),
            day: date.next().unwrap().parse().unwrap(),
            hour: time.next().unwrap().parse().unwrap(),
            minute: time.next().unwrap().parse().unwrap(),
            second: time.next().unwrap().parse().unwrap()
        })
    }
}

impl fmt::Display for StrDateTime{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}-{}-{} {}:{}:{}",
            self.year,
            self.month,
            self.day,
            self.hour,
            self.minute,
            self.second
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance(){
        let first: StrDateTime = ("2001-02-18 10:00:00").parse().unwrap();
        let second: StrDateTime = ("1997-07-12 10:00:00").parse().unwrap();
        assert_eq!(first.distance_between(second).days, 1317);
        assert_eq!(first.distance_between(second).sundays, 189);
        assert_eq!(first.distance_between(second).saturdays, 189);
    }

    #[test]
    fn add_distance(){
        let date: StrDateTime = ("2023-01-12 00:00:00").parse().unwrap();
        assert_eq!(date.add_seconds(86400), StrDateTime::new(2023, 01, 13, 0,0,0));
        assert_eq!(date.add_minutes(10), StrDateTime::new(2023, 01, 12, 0,10,0));
        assert_eq!(date.add_hours(1), StrDateTime::new(2023, 01, 12, 01,0,0));
        assert_eq!(date.add_days(1), StrDateTime::new(2023, 01, 13, 0,0,0));
        assert_eq!(date.add_weeks(1), StrDateTime::new(2023, 01, 19, 0,0,0));
        assert_eq!(date.add_years(1), StrDateTime::new(2024, 01, 12, 0,0,0));
    }

    #[test]
    fn display_now(){
        let date = StrDateTime::now();
        assert_eq!(date.year, 2023);
    }
    
}
