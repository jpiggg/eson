use wasm_bindgen::prelude::*;
use js_sys;
use crate::parse::parser::ast;

pub fn to_object(pairs: &Vec<ast::Pair>) -> js_sys::Object {
    let object = js_sys::Object::new();

    for pair in pairs.iter() {
        let key_extracted = match &pair.key {
            ast::Key::KeySimple(s) => s.value,
            ast::Key::KeyComplex(s) => s.value,
        };
        let key: js_sys::JsString = js_sys::JsString::from(key_extracted);
        let _ = js_sys::Reflect::set(&object, &key, &compile_value(&pair.value));
    }

    object
}

pub fn to_array(items: &Vec<ast::Value>) -> js_sys::Array {
    let result = js_sys::Array::new();

    for item in items.iter() {
        result.push(&compile_value(item));
    }

    result
}

pub fn compile_value(value: &ast::Value) -> JsValue {
    match value {
        ast::Value::Object(obj) => {
            to_object(&obj.pair).into()
        },
        ast::Value::Array(arr) => {
            to_array(&arr.value).into()
        },
        ast::Value::String(s) => {
            js_sys::JsString::from(s.value).into()
        },
        ast::Value::Number(n) => {
            js_sys::Number::from(n.value).into()
        },
        ast::Value::Boolean(b) => {
            JsValue::from(b.value)
        },
        ast::Value::Null(_) => {
            JsValue::null()
        },
        ast::Value::Undefined(_) => {
            JsValue::undefined()
        },  
        ast::Value::NaN(_) => {
            JsValue::from(f64::NAN)
        },
        ast::Value::Infinity(i) => {
            if i.value < 0 {
                js_sys::Number::NEGATIVE_INFINITY.into()
            } else {
                js_sys::Number::POSITIVE_INFINITY.into()
            }
        },
        ast::Value::BigInt(bi) => {
            JsValue::bigint_from_str(&bi.value[0..bi.value.len() - 1])
        }
    }
}

pub fn compile(eson: &ast::ESon) -> JsValue {
    to_object(&eson.object.pair).into()
}
