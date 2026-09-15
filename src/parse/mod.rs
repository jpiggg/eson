
pub mod parser;
pub mod compiler;

use self::parser::ast::ESon;
use self::compiler::compile;
use from_pest::FromPest;
use pest::Parser;

use js_sys::JsString;
use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub fn parse(source: JsString) -> JsValue {
    let raw_source= &source.as_string().unwrap();
    let mut parse_tree = self::parser::ESonParser::parse(self::parser::Rule::ESon, &raw_source).unwrap();
    let syntax_tree: ESon = ESon::from_pest(&mut parse_tree).expect("infallible");
    
    let result: wasm_bindgen::JsValue = compile(&syntax_tree);
    
    result
}