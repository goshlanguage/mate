use tda_sdk::responses::Candle;

// https://www.investopedia.com/terms/e/ema.asp
// symbol - ticker symbol of the security you want to query
// period - number in days to grab info for
//
// You can cross check https://www.tradingview.com/symbols/$exchange-$symbol/technicals/ for validity/manual checking
// eg: https://www.tradingview.com/symbols/NASDAQ-MSFT/technicals/
//
// TODO Fix no boundary checking
// TODO check for NaN
// returns computed ema as f64
pub fn ema(candles: &[Candle], period: i32) -> f64 {
    let sma_i = (period) as usize;
    let sma_e = (2 * period) as usize;
    let base_case = sma(candles, sma_i as usize, sma_e as usize);

    let len_candles = candles.len();
    let close = candles[len_candles - (period as usize)].close;
    let smoothing_factor = 2.0;
    let multiplier = smoothing_factor / (1.0 + f64::from(period));
    let ema0 = round((close - base_case) * multiplier + base_case);

    let mut emas = vec![ema0];
    // EMA = Closing price x multiplier + EMA (previous day) x (1-multiplier)
    for i in 0..period-1 {
        let len_candles = candles.len();
        let close = candles[len_candles - ((period - i) as usize)].close;
        let previous_ema = emas[i as usize];

        let ema_i = round((close - previous_ema) * multiplier + previous_ema);
        emas.push(ema_i);
    }

    emas[emas.len() - 1]
}

pub fn sma(candles: &[Candle], start: usize, period: usize) -> f64 {
    let mut sum = 0.0;

    for i in start..start + period {
        let i_usize = i as usize;
        sum += candles[candles.len() - 1 - i_usize].close;
    }

    let average = sum / (period as f64);

    round(average)
}

// round is a helper for f64 that rounds the number to a decimal point notation used for representing money
fn round(i: f64) -> f64 {
    (i * 100.0).round() / 100.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use tda_sdk::responses::GetPriceHistoryResponse;

    #[test]
    fn sma_test() {
        let candles = &test_helper();
        let sma = sma(candles, 0, 3);
        assert_eq!(sma, 10.76);
    }

    #[test]
    fn round_test() {
        let input = 1.23456789;
        let rounded = round(input);

        assert_eq!(rounded, 1.23);

        let input = 3.456789;
        let rounded = round(input);

        assert_eq!(rounded, 3.46);

        let input = 4.56789;
        let rounded = round(input);

        assert_eq!(rounded, 4.57);
    }

    fn test_helper() -> Vec<Candle> {
        let sample_ticker_data = r#"{
            "candles": [
                {
                    "open": 10.525,
                    "high": 11.395,
                    "low": 10.34,
                    "close": 12,
                    "volume": 54379431,
                    "datetime": 1606284000000
                },
                {
                    "open": 9.35,
                    "high": 10.75,
                    "low": 9.32,
                    "close": 10.41,
                    "volume": 72355931,
                    "datetime": 1606284000000
                },
                {
                    "open": 11.04,
                    "high": 11.21,
                    "low": 10.63,
                    "close": 10.86,
                    "volume": 57905144,
                    "datetime": 1606197600000
                },
                {
                    "open": 10.525,
                    "high": 11.395,
                    "low": 10.34,
                    "close": 11,
                    "volume": 54379431,
                    "datetime": 1606284000000
                }
              ],
              "symbol": "M",
              "empty": false
            }
        "#;

        let sample: GetPriceHistoryResponse = serde_json::from_str(sample_ticker_data).unwrap();

        sample.candles
    }
}
