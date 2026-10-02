use crate::stocks::{Stock, StocksDb};
mod stocks;
mod stock_data;
use chrono_tz::Asia::Jerusalem;
use yfinance_rs::{Interval, Range, Ticker, YfClient};


async  fn get_ticker_info(yf_client: &YfClient, symbol: &str, range: Range, interval: Interval) -> Result<(),Box<dyn std::error::Error>>
{
    let ticker = Ticker::new(yf_client, symbol);
    let history = ticker.history(Some(range), Some(interval), false).await?;

    for item in &history
    {
        println!("{:?}:{:?}", item.ts.with_timezone(&Jerusalem).time(),item.close_unadj);
    }
    if let Some(last_bar) = history.last() {
        println!("Last closing price: {} on timestamp {}", last_bar.ohlc.close, last_bar.ts);
    }

    Ok(())
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_str = include_str!("../clean_tickers.json");
    let stocks: Vec<Stock> = serde_json::from_str(db_str).expect("failed to create stocks data");
    let stock_db = StocksDb::new(&stocks);

    let client = YfClient::default();
    let ticker = Ticker::new(&client, "AAPL");

    // Get the latest quote
    let quote = ticker.quote().await?;
    if let Some(price) = quote.price.as_ref() {
        println!("Latest price for AAPL: {price}");
    }

    // Get historical data for the last 6 months
    get_ticker_info(&client,"AAPL", Range::D1, Interval::I5m)
        .await?;


    // Get analyst recommendations
    let recs = ticker.recommendations().await?;
    if let Some(latest_rec) = recs.first() {
        println!("Latest recommendation period: {}", latest_rec.period);
    }


    Ok(())
}
