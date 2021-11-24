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
    for i in 0..period {
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

    for i in start..end {
        let i_usize = i as usize;
        sum += candles[candles.len() - 1 - i_usize].close;
    }

    let difference = (end - start) as f64;
    let average = sum / difference;

    round(average)
}

// round is a helper for f64 that rounds the number to a decimal point notation used for representing money
fn round(i: f64) -> f64 {
    (i * 100.0).round() / 100.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sma_test() {

      let candle_1 = Candle{
        close: 1.0,
        datetime: 12345,
        high: 0.0,
        low: 0.0,
        open: 1.0,
        volume: 1,
      };

      let candle_2 = Candle{
        close: 2.0,
        datetime: 12345,
        high: 0.0,
        low: 0.0,
        open: 1.0,
        volume: 1,
      };

      let candle_3 = Candle{
        close: 3.0,
        datetime: 12345,
        high: 0.0,
        low: 0.0,
        open: 1.0,
        volume: 1,
      };

      let sma = sma(&[candle_1, candle_2, candle_3], 0, 3);
      assert_eq!(sma, 2.0);
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
}
