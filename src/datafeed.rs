use tda_sdk::{
  params::GetMoversParams, Client,
};

pub fn get_movers(client: Client) {
    // let movers_params = GetMoversParams{
    //     change: Some("up".to_string()),
    //     direction: Some("percent".to_string()),
    // };

    let default = GetMoversParams::default();

    let index = "$DJI";
    let movers = client.get_movers(index, default).unwrap();

    println!("Current movers: {:?}", movers);
}
