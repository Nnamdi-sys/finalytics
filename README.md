## Finalytics

**Finalytics** is a Rust library designed for retrieving financial data and performing various financial analysis tasks, including fundamental analysis, technical analysis, sentiment analysis, options pricing, portfolio optimization, and displaying financial charts. The library fetches data from Yahoo Finance.

## Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
finalytics = "0.1.0"
```

You would need to install the following packages due to dependencies from the ejdb and rust-bert libraries:

```
pkg-config zlib cmake gcc clang
```

linux:
```bash
sudo apt-get update
sudo apt-get install pkg-config libssl-dev zlib1g-dev cmake gcc clang
```

macOS:
```bash
brew update
brew install pkg-config openssl zlib cmake gcc clang
````

windows:
```bash
choco install pkgconfiglite openssl zlib cmake gcc clang
```

### Documentation

###### You can access the user guide by visiting [finalytics.fly.dev](https://finalytics.fly.dev/)
###### You can access the crates.io documentation by visiting [docs.rs/finalytics](https://docs.rs/finalytics/)


### Sample Applications

#### Ticker Chart Viewer

You can access the Ticker Chart Viewer by visiting [finalytics.fly.dev/ticker](https://finalytics.fly.dev/ticker). This sample application allows you to perform security analysis based on the Finalytics Library.

#### Portfolio Chart Viewer

You can access the Portfolio Chart Viewer by visiting [finalytics.fly.dev/portfolio](https://finalytics.fly.dev/portfolio). This sample application enables you to perform portfolio optimization based on the Finalytics Library.


