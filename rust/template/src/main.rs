use regex::Regex;

use serde::{Deserialize, Serialize};
use serde_json::Result;


mod html_list;
use html_list::htmls::{HOME};


#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}



fn typed_example() -> Result<()> {    
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    let p: Person = serde_json::from_str(data)?;
    
    println!("Please call {} at the number {}", p.name, p.phones[0]);

    Ok(())
}



fn main() {

    typed_example().unwrap();

    
    let re = Regex::new(r"\{\{=([\s\S]+?)\}\}").unwrap();
    
    if re.is_match(HOME) {

        for caps in re.captures_iter(HOME) {
            println!("Capture: {}", &caps[0]);
        }

    }

    println!("Home page match: {}", re.is_match(HOME));

    println!("home page: {}", HOME);
}