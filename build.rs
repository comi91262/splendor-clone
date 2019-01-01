extern crate failure;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate lazy_static;

use serde_json::Map;
use serde_json::Value;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;

lazy_static! {
    static ref CREATED_FILE_PATH: String = {
        let working_directory = env!("CARGO_MANIFEST_DIR");
        let file_path = "card.json";
        format!("{}/{}", working_directory, file_path)
    };
}

#[derive(Serialize, Deserialize, Debug)]
enum Color {
    Black,
    White,
    Red,
    Blue,
    Green,
}

#[derive(Serialize, Deserialize, Debug)]
struct Card {
    level: u8,
    color: Color,
    point: u8,
    cost_black: u8,
    cost_white: u8,
    cost_red: u8,
    cost_blue: u8,
    cost_green: u8,
}

//fn parse_json(data: &str) -> Result<String, failure::Error> {
//    let mut result_str = String::new();
//    let json: HtmlEntity = serde_json::from_str(data)?;
//
//    for (entity_name, data) in json.nodes {
//        if entity_name.ends_with(";") {
//            let mut characters = String::new();
//            for c in data.characters.chars() {
//                characters.push_str(&format!("{}", c.escape_unicode()));
//            }
//            result_str.push_str(&format!(
//                "(\"{}\", \"{}\"),\n",
//                re.replace_all(&entity_name, ""),
//                characters
//            ));
//        }
//    }
//
//    Ok(result_str)
//}
//
fn main() {
    let data = r#"{
                    "level": 1,
                    "color": "Black",
                    "point": 0,
                    "cost_black": 0,
                    "cost_white": 0,
                    "cost_red": 0,
                    "cost_blue": 0,
                    "cost_green": 0
                  }"#;

    let card: Card = serde_json::from_str(data).unwrap();

    // let json = format!("{{\"nodes\": {} }}", std::str::from_utf8(&card).unwrap());

    // let parsed_str = parse_json(&json).unwrap();

    //let result = format!("{}{}];", BOILERPLATE, parsed_str);

    //    let mut f = File::create(&*CREATED_FILE_PATH).unwrap();
    //   f.write_all(json.as_bytes()).unwrap();
}
