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
    pub fn new(stocks: &Vec<Stock>) -> Self {
        StocksDb {
            stocks: (*stocks).to_vec(),
        }
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
