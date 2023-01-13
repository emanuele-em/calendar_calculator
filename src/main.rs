use calendar_calculator::CalendarCalculator;

fn main() {
    let first = "2004-02-29 10:00:00";
    let second = "1997-07-12 10:00:00";
    println!("{}", first.distance_between(second));
    let (y, m, d, _, _, _) = first.add_days(-2423);
    println!("{}-{}-{}", y, m, d);
}
