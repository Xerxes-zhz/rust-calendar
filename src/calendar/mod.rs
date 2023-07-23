use time::Date;
mod dates;
use dates::Dates;
use time::error::ComponentRange;
pub fn date_range(start: Date, end: Date) -> Result<Vec<Date>, ComponentRange> {
    (start.to_julian_day()..=end.to_julian_day())
        .map(|day| Date::from_julian_day(day))
        .collect()
}

pub fn next_day<T: Dates<T>>(dates: T) -> T::OutputDate {
    dates.next_nth_day(1)
}
pub fn prev_day<T: Dates<T>>(dates: T) -> T::OutputDate {
    dates.prev_nth_day(1)
}
pub fn next_n_day<T: Dates<T>>(dates: T, n: i32) -> T::OutputDate {
    dates.next_nth_day(n)
}
pub fn prev_n_day<T: Dates<T>>(dates: T, n: i32) -> T::OutputDate {
    dates.prev_nth_day(n)
}
