//! Implementations for `Exchange` types.
use crate::{types::*};

impl From<String> for Exchange {
	fn from(s: String) -> Self {
		match s.as_str() {
			"bitflyer" => Exchange::BitFlyer,
			"bittrex" => Exchange::Bittrex,
			"gemini" => Exchange::Gemini,
			"luno" => Exchange::Luno,
			"gateio" => Exchange::Gateio,
			"bitfinex" => Exchange::Bitfinex,
			"kraken" => Exchange::Kraken,
			"cexio" => Exchange::Cexio,
			"bisq" => Exchange::Bisq,
			"bitmex" => Exchange::BitMEX,
			"okex" => Exchange::Okex,
			"kraken-futures" => Exchange::KrakenFutures,
			"liquid" => Exchange::Liquid,
			"quoine" => Exchange::Quoine,
			"bitbay" => Exchange::BitBay,
			"hitbtc" => Exchange::HitBTC,
			"binance" => Exchange::Binance,
			"binance-us" => Exchange::BinanceUS,
			"huobi" => Exchange::Huobi,
			"poloniex" => Exchange::Poloniex,
			"coinbase-pro" => Exchange::CoinbasePro,
			"bitstamp" => Exchange::Bitstamp,
			"bitz" => Exchange::BitZ,
			"bithumb" => Exchange::Bithumb,
			"coinone" => Exchange::Coinone,
			"dex-aggregated" => Exchange::DexAggregated,
			"okcoin" => Exchange::OkCoin,
			"ftx" => Exchange::Ftx,
			"uniswap-v2" => Exchange::UniswapV2,
			"bybit" => Exchange::Bybit,
			"crypto-com" => Exchange::CryptoCom,
			"deribit" => Exchange::Deribit,
			"kucoin" => Exchange::KuCoin,
			"okx" => Exchange::Okex,
			"zonda" => Exchange::Zonda,
			"wex" => Exchange::Wex,
			_ => Exchange::Other(s.to_string()),
		}
	}
}

impl<'a> From<&'a str> for Exchange {
	fn from(s: &'a str) -> Self {
		match s {
			"bitflyer" => Exchange::BitFlyer,
			"bittrex" => Exchange::Bittrex,
			"gemini" => Exchange::Gemini,
			"luno" => Exchange::Luno,
			"gateio" => Exchange::Gateio,
			"bitfinex" => Exchange::Bitfinex,
			"kraken" => Exchange::Kraken,
			"cexio" => Exchange::Cexio,
			"bisq" => Exchange::Bisq,
			"bitmex" => Exchange::BitMEX,
			"okex" => Exchange::Okex,
			"kraken-futures" => Exchange::KrakenFutures,
			"liquid" => Exchange::Liquid,
			"quoine" => Exchange::Quoine,
			"bitbay" => Exchange::BitBay,
			"hitbtc" => Exchange::HitBTC,
			"binance" => Exchange::Binance,
			"binance-us" => Exchange::BinanceUS,
			"huobi" => Exchange::Huobi,
			"poloniex" => Exchange::Poloniex,
			"coinbase-pro" => Exchange::CoinbasePro,
			"bitstamp" => Exchange::Bitstamp,
			"bitz" => Exchange::BitZ,
			"bithumb" => Exchange::Bithumb,
			"coinone" => Exchange::Coinone,
			"dex-aggregated" => Exchange::DexAggregated,
			"okcoin" => Exchange::OkCoin,
			"ftx" => Exchange::Ftx,
			"uniswap-v2" => Exchange::UniswapV2,
			"bybit" => Exchange::Bybit,
			"crypto-com" => Exchange::CryptoCom,
			"deribit" => Exchange::Deribit,
			"kucoin" => Exchange::KuCoin,
			"okx" => Exchange::Okex,
			"zonda" => Exchange::Zonda,
			"wex" => Exchange::Wex,
			_ => Exchange::Other(s.to_string()),
		}
	}
}
