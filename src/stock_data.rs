use crate::stocks::Stock;
use chrono::DateTime;
use chrono_tz::Tz;
#[allow(dead_code)]
pub struct DataPoint {
    pub point: (DateTime<Tz>, f64),
}
#[allow(dead_code)]
pub struct StockData {
    pub stock: Stock,
    pub latest_price: f64,
    pub daily: Vec<DataPoint>,
    pub five_days: Vec<DataPoint>,
    pub monthly: Vec<DataPoint>,
    pub half_yearly: Vec<DataPoint>,
    pub yearly: Vec<DataPoint>,
}
