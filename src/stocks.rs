use std::cmp::Ordering;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Stock
{
    symbol: String,
    name: String,
    is_etf : bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StocksDb
{
    pub stocks : Vec<Stock>
}

impl StocksDb
{
    pub fn new(stocks: &Vec<Stock>) -> Self
    {
        StocksDb
        {
            stocks: (*stocks).to_vec()
        }
    }
    pub fn search_by_symbol(&self, symbol : &str) -> Option<Stock>
    {
        let size = self.stocks.len();
        let mut high  = size ;
        let mut low = 0;
        let mut middle = high /2;
        while  high > low {

            let current = &self.stocks[middle];
            match current.symbol.cmp(&symbol.to_string())
            {
                Ordering::Equal => return Some(current.clone()),
                Ordering::Greater =>
                    {
                        high = middle +1;
                        middle = (low + high) / 2;


                    },
                Ordering::Less =>
                    {
                        low = middle ;
                        middle = (low + high) / 2;


                    }
            }
        }
        None
    }
    pub fn search_by_name(&self, name: &str) ->Option<Stock>
    {
        let name_lower = name.to_lowercase();
        for stock in &self.stocks
        {
            if stock.name.to_lowercase().contains(&name_lower)
            {
                return Some(stock.clone());
            }
        }
        None
    }
}