use time::Date;
pub mod dates;
use dates::Dates;
use dates::DATE_FORMAT;
use time::error::ComponentRange;
pub struct TradingList {
    trading: Vec<i32>,
    db_days: Vec<i32>,
    base: i32,
    last: i32,
}
impl TradingList {
    pub fn new(dates: Vec<i32>, first: i32) -> Self {
        let last: i32 = dates.get(dates.len() - 1).unwrap().clone();
        let mut day: usize = 0usize;
        let mut days: Vec<i32> = Vec::new();
        for (i, val) in dates.iter().enumerate() {
            if day == *val as usize {
                days.push(i as i32 * 2);
                day += 1;
            } else if day < *val as usize {
                for _ in (day..*val as usize) {
                    days.push((i * 2 - 1) as i32);
                }
                days.push(i as i32 * 2);

                day = *val as usize + 1usize;
            }
        }
        Self {
            trading: dates,
            db_days: days,
            base: first,
            last: last,
        }
    }
    // from 0
    pub fn get_next(&self, date: Date, nth: i32) -> Date {
        let n: i32 = date.to_julian_day() - self.base;
        let id: &i32 = self.db_days.get(n as usize).unwrap();
        let date: &i32 = self.trading.get(((id + 1) / 2 + nth) as usize).unwrap();
        Date::from_julian_day(date + self.base).unwrap()
    }
    // from 0
    pub fn get_prev(&self, date: Date, nth: i32) -> Date {
        let n: i32 = date.to_julian_day() - self.base;
        let id: &i32 = self.db_days.get(n as usize).unwrap();
        let date: &i32 = self.trading.get(((id) / 2 - nth) as usize).unwrap();
        Date::from_julian_day(date + self.base).unwrap()
    }
}

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

pub fn string_to_julian(dates: Vec<String>) ->TradingList{
    let first_julian: i32 = Date::parse(dates.get(0).unwrap(), DATE_FORMAT)
        .unwrap()
        .to_julian_day();
    let dates: Vec<i32> = dates
        .iter()
        .map(|date| Date::parse(date, DATE_FORMAT).unwrap().to_julian_day() - first_julian)
        .collect();
    TradingList::new(dates, first_julian)

}
