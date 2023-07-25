use time::Date;
use time::Duration;
use time::macros::format_description;
use time::format_description::FormatItem;
#[derive(Debug)]
pub enum DateError{
    InvalidDate,
    CalculationError(String),
}
pub const DATE_FORMAT:&[FormatItem<'_>]= format_description!("[year][month][day]");
pub trait Dates<T> {
    type OutputDate;
    fn next_nth_day(self, n: i32) -> Self::OutputDate;
    fn prev_nth_day(self, n: i32) -> Self::OutputDate;
}
impl Dates<Date> for Date {
    type OutputDate = Option<Date>;

    fn next_nth_day(self, n: i32) -> Self::OutputDate {
        if n == 1 {
            self.next_day()
        } else {
            self.checked_add(Duration::days(n as i64))
        }
    }
    fn prev_nth_day(self, n: i32) -> Self::OutputDate {
        if n == 1 {
            self.previous_day()
        } else {
            self.checked_sub(Duration::days(n as i64))
        }
    }
}

impl Dates<Vec<Date>> for Vec<Date> {
    type OutputDate = Result<Vec<Date>,DateError>;

    fn next_nth_day(self, n: i32) -> Self::OutputDate {
        self.iter()
            .try_fold(vec![], |mut acc, date|{
                match date.checked_add(Duration::days(n as i64)) {
                    Some(new_date) =>{
                        acc.push(new_date);
                        Ok(acc)
                    },
                    None => Err(DateError::CalculationError(format!("Invalid date: {:?}",date))),
                    
                }
            })
    }
    fn prev_nth_day(self, n: i32) -> Self::OutputDate {
        self.iter()
            .try_fold(vec![], |mut acc, date|{
                match date.checked_sub(Duration::days(n as i64)) {
                    Some(new_date) =>{
                        acc.push(new_date);
                        Ok(acc)
                    },
                    None => Err(DateError::CalculationError(format!("Invalid date: {:?}",date))),
                    
                }
            })
    }
}
