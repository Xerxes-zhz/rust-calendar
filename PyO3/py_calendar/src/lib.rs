use pyo3::prelude::*;
use time::Date;
use time::error::ComponentRange;
use time::macros::format_description;
fn date_range(start: Date, end: Date) -> Result<Vec<Date>, ComponentRange> {
    (start.to_julian_day()..=end.to_julian_day())
        .map(|day| Date::from_julian_day(day))
        .collect()
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}
#[pyfunction]
fn date_range_rs(start:String, end:String)->PyResult<Vec<String>>{
    let date_format = format_description!("[year][month][day]");
    let start:Date = Date::parse(&start, &date_format).unwrap();
    let end:Date = Date::parse(&end, &date_format).unwrap();

    Ok(match date_range(start,end){
        Ok(n) =>n.iter().map(|day|
            day.format(&date_format).unwrap()).collect(),
        Err(_)=>panic!("wrong")
    })
}

/// A Python module implemented in Rust.
#[pymodule]
fn py_calendar(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(date_range_rs, m)?)?;
    Ok(())
}