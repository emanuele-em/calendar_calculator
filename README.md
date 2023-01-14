# Calendar Calculator

The calendar_calculator crate is a simple library crate that provides a range of functionalities for calculating distances between two dates expressed in str slice format, as well as adding different durations to a given date. With this crate, users can easily and quickly perform calculations related to dates and distances.

## Installation

This crate works with Cargo. Add the following to your Cargo.toml dependencies section:

```
[dependencies]
calendar_calculator = "0.2.0"
```
or simply run
```
cargo add calendar_calculator
```

## Usage

```rust
use calendar_calculator::CalendarCalculator



let n = 10;
let date = ("2023-01-12 00:00:00")
    .parse().unwrap(); // FromStr Trait Implemented

let now = StrDateTime::now(); // return now in StrDateTime format

println!("{}", date.add_seconds(n)); // 2001-02-18 10:00:10

println!("{}", date.add_minutes(n)); // 2001-02-18 10:10:00

println!("{}", date.add_hours(n)); // 2001-02-18 20:10:00

println!("{}", date.add_days(n)); // 2001-02-28 10:00:00

println!("{}", date.add_weeks(n)); // 2001-04-29 10:00:00

println!("{}", date.add_months(n)); // 2001-12-18 10:00:00

println!("{}", date.add_years(n)); // 2011-02-18 10:00:00

println!("{}", date.add_years(n)); // 2011-02-18 10:00:00

println!({},("1997-07-12 10:00:00").parse().unwrap().distance_between(date));
// {
//  seconds: 113788800,
//  minutes: 1896480,
//  hours: 31608,
//  days: 1317,
//  weeks: 188,
//  months: 43,
//  years: 3,
//  sundays: 189,
//  saturdays: 189,
//  working_days: 1128,
// }
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

[MIT](https://choosealicense.com/licenses/mit/) or
[Apache License 2.0](https://choosealicense.com/licenses/apache-2.0/)
