use anyhow::Error;

use time::macros::date;
// use time::macros::format_description;
use time::Date;
mod calendar;
mod database;
use calendar::dates::DATE_FORMAT;
use std::time::Instant;

fn main() {
    let trading_day:Vec<String> = database::link().unwrap();
    let tl = calendar::string_to_julian(trading_day);
    let start= Instant::now();
    // tl.get_next(Date::parse("20230723", DATE_FORMAT).unwrap(), 3);
    let range = calendar::date_range(date!(2023-01-01), date!(2024-01-01)).unwrap();
    let next3day: Vec<Date> = range.iter().map(move |&date| tl.get_next(date, 3)).collect();
    let end= Instant::now();
    println!("{:?}",next3day);
    println!("{}",(end-start).as_micros());

}
