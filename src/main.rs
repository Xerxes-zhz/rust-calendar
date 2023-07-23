use time::macros::date;
mod calendar;

fn main() {


    let one_day = date!(2023-01-01);
    let range_day = calendar::date_range(date!(2023-01-01),date!(2023-07-15)).unwrap();
    println!("{}",calendar::next_day(one_day).unwrap());
    // println!("{:?}",calendar::next_day(range_day));
    println!("{:?}",calendar::next_n_day(range_day,1000000000));
    println!("{}",calendar::prev_day(one_day).unwrap());
}
