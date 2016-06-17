extern crate rustc_serialize;

use self::rustc_serialize::json;


#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct Config {
    pub env: &'static str,
    pub enable: bool,
}

pub fn run() {
    let c = Config {
        env: "production",
        enable: false,
    };

    println!("Default Cofnig: {:?}", c);

    let encoded = json::encode(&c).unwrap();

    println!("Default encoded config: {}", encoded);

}
