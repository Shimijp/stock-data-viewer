# Stock Data Viewer

[![CI](https://github.com/Shimijp/stock-data-viewer/actions/workflows/ci.yml/badge.svg)](https://github.com/Shimijp/stock-data-viewer/actions/workflows/ci.yml)

A desktop app for analyzing and displaying stock and fund data locally, in the spirit of Google Finance. Built with Rust, with an [egui](https://github.com/emilk/egui) UI planned.

Market data comes from Yahoo Finance via [`yfinance-rs`](https://crates.io/crates/yfinance-rs). Ticker search is offline, using a bundled list of about 13,000 US-listed stocks and ETFs.

> **Status:** early development. It's currently a command-line demo. See the [milestones](https://github.com/Shimijp/stock-data-viewer/milestones) and [issues](https://github.com/Shimijp/stock-data-viewer/issues) for the roadmap.

## Getting started

Requires a recent stable Rust toolchain (edition 2024, so Rust 1.85+).

```sh
cargo run
```

## Development

Run these before pushing. CI runs the same checks and must pass before a PR can merge:

```sh
cargo fmt
cargo clippy --all-targets -- -D warnings
cargo test
```

Workflow:
1. Pick an open [issue](https://github.com/Shimijp/stock-data-viewer/issues).
2. Branch off `master`: `git checkout -b <type>/<issue>-short-name` (e.g. `feat/8-yahoo-provider`).
3. Open a PR with `Closes #<issue>` in the description.
4. When CI is green and the PR is approved, squash and merge.

## Regenerating the ticker list

`clean_tickers.json` is generated from Nasdaq's public symbol directory and compiled into the binary. To refresh it:

```sh
pip install pandas
python create_db.py
```

The script sorts entries by symbol and drops duplicate symbols. `StocksDb` also sorts on load, so symbol lookup (binary search) stays correct either way.

## Project layout

| Path | Purpose |
|---|---|
| `src/main.rs` | Entry point (CLI demo for now) |
| `src/stocks.rs` | Ticker database and search |
| `src/stock_data.rs` | Price data types (to be replaced by `model.rs` in M1) |
| `create_db.py` | Builds `clean_tickers.json` |
| `.github/workflows/` | CI and Claude Code automation |
