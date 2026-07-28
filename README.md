![Rust](https://img.shields.io/badge/Rust-1.90+-orange)
![GUI](https://img.shields.io/badge/GUI-egui-blue)
![Status](https://img.shields.io/badge/status-Active-success)

# ₿ Rust Crypto Analyzer

A desktop cryptocurrency market tracker built with **Rust**, **egui**, and the **CoinGecko API**.

This project provides a clean and responsive desktop interface for monitoring cryptocurrency market data in real time. Users can search, sort, filter, favorite coins, view detailed market information, and automatically refresh market data through configurable settings.

## ✨ Features

- 📊 Real-time cryptocurrency market data
- 📋 Professional table layout
- 🔢 Configurable number of displayed coins
- 🔍 Search by coin name or symbol
- 📈 Sorting by:
  - Market Rank
  - Price
  - Market Cap
  - Trading Volume
  - 24h Price Change
- ⭐ Favorite coins with persistent local storage
- ❤️ Show only favorite coins
- 📄 Coin details page
- 🔄 Manual refresh
- ⚙️ Configurable auto refresh
- 💲 Smart price formatting
- 💾 Favorites stored locally in JSON
- 🚨 Error handling with temporary notifications

## 🛠 Built With

- Rust
- egui / eframe
- Tokio
- Reqwest
- Serde 
- Serde JSON
- Chrono

## 🚀 Running

```bash
cargo run
```

## 🎯 Project Goals

This project was developed to practice:

- Modular project architecture
- Async programming with Tokio
- REST API integration
- Desktop GUI development using egui
- Application state management
- Error handling
