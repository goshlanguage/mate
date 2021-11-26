use tda_sdk::{params::GetMoversParams, Client};
use std::{time::{Duration, SystemTime, UNIX_EPOCH}};

pub fn print_movers(client: Client) {
    // let movers_params = GetMoversParams{
    //     change: Some("up".to_string()),
    //     direction: Some("percent".to_string()),
    // };

    let default = GetMoversParams::default();

    let index = "$DJI";
    let movers = client.get_movers(index, default).unwrap();

    println!("Current movers: {:?}", movers);
}

fn get_epoch_after_period(period: i64) {
    let day_in_ms = 24 * 60 * Duration::new(60, 0).as_millis();
    let period_in_ms = (period as u128) * day_in_ms;

    let now = SystemTime::now();
    let start_epoch = now
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    let end_epoch = start_epoch - period_in_ms;

    println!("start: {:?}\tend: {:?}", start_epoch, end_epoch);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn epoch_test() {
      get_epoch_after_period(20);
  }
}
