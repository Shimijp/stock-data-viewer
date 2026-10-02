use crate::stocks::Stock;
use chrono::{DateTime, NaiveDate, NaiveTime};
use chrono_tz::Tz;

pub struct DataPoint
{
    pub point: (DateTime<Tz>, f64)
}

pub struct StockData {
    pub stock: Stock,
    pub latest_price: f64,
    pub daily: Vec<DataPoint>,
    pub five_days: Vec<DataPoint>,
    pub monthly: Vec<DataPoint>,
    pub half_yearly: Vec<DataPoint>,
    pub yearly: Vec<DataPoint>,
}