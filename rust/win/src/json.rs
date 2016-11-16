
use super::rustc_serialize::json;

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct Config {
    env: String,
    enable: bool,
    data_vectro: Vec<u8>,

}

pub fn run() {
    let c = Config {
        env: "production".to_string(),
        enable: false,
        data_vectro: vec![1,2,3,4],
    };

    println!("Default Cofnig: {:?}", c);

    let encoded = json::encode(&c).unwrap();

    println!("Default encoded config: {}", encoded);

}
