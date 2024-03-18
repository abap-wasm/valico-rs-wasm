use wasm_bindgen::prelude::*;
use valico::json_schema;
use serde_json::json;

#[wasm_bindgen]
pub fn main() -> String {
    // let json_schema: Value = serde_json::from_reader(File::open("tests/schema/schema.json").unwrap()).unwrap();
    let json_schema = json!({"maxLength": 5});
    let json = json!("fosssso");

    let mut scope = json_schema::Scope::new();
    let schema = scope.compile_and_return(json_schema.clone(), false).unwrap();

    let mut ret: String = "".to_owned();
//    println!("Is valid: {}", schema.validate(&json).is_valid());

    let valid = schema.validate(&json).is_valid();
    ret.push_str(format!("Is valid: {valid}").as_str());

    return ret;
}