import pandas as pd

url = "ftp://ftp.nasdaqtrader.com/SymbolDirectory/nasdaqtraded.txt"

df = pd.read_csv(url, sep="|")

df = df.dropna(subset=['Symbol'])

df_clean = df[['Symbol', 'Security Name', 'ETF']].copy()

df_clean.columns = ['symbol', 'name', 'is_etf']


df_clean['is_etf'] = df_clean['is_etf'] == 'Y'

df_clean = df_clean.drop_duplicates()

df_sorted = df_clean.sort_values(by='symbol')

df_sorted.to_json("clean_tickers.json", orient="records")

print(f"Successfully saved {len(df_clean)} assets to clean_tickers.json")