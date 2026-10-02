use serde::{Deserialize, Serialize};
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Stock {
    symbol: String,
    name: String,
    is_etf: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StocksDb {
    pub stocks: Vec<Stock>,
}
#[allow(dead_code)]
impl StocksDb {
    pub fn new(mut stocks: Vec<Stock>) -> Self {
        stocks.sort_by(|a, b| a.symbol.cmp(&b.symbol));
        StocksDb { stocks }
    }
    pub fn search_by_symbol(&self, symbol: &str) -> Option<&Stock> {
        self.stocks
            .binary_search_by(|s| s.symbol.as_str().cmp(symbol))
            .ok()
            .map(|i| &self.stocks[i])
    }
    pub fn search_by_name(&self, name: &str) -> Option<&Stock> {
        let name_lower = name.to_lowercase();
        self.stocks
            .iter()
            .find(|&stock| stock.name.to_lowercase().contains(&name_lower))
            .map(|v| v as _)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_sample_db() -> StocksDb {
        let stocks_json = r#"[
    {
        "symbol": "AAPL",
        "name": "Apple Inc.",
        "is_etf": false
    },
    {
        "symbol": "MSFT",
        "name": "Microsoft Corporation",
        "is_etf": false
    },
    {
        "symbol": "SPY",
        "name": "SPDR S&P 500 ETF Trust",
        "is_etf": true
    },
    {
        "symbol": "QQQ",
        "name": "Invesco QQQ Trust",
        "is_etf": true
    }
]"#;
        let stocks: Vec<Stock> = serde_json::from_str(stocks_json).expect("failed to load stocks");
        let db = StocksDb::new(stocks);
        db
    }
    #[test]
    fn finds_symbol() {
        let db = create_sample_db();
        assert!(db.search_by_symbol("AAPL").is_some());
    }
    #[test]
    fn non_exiting_symbol() {
        let db = create_sample_db();
        assert!(db.search_by_symbol("ZZZZ").is_none())
    }
    #[test]
    fn find_last_symbol() {
        let db = create_sample_db();
        assert!(db.search_by_symbol("QQQ").is_some());
    }
    #[test]
    fn find_by_name_lower() {
        let db = create_sample_db();
        assert!(db.search_by_name("apple").is_some())
    }
}
