use js_sys::{self, BigInt};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn stringify(source: &JsValue) -> js_sys::JsString {
    return match source {
        source if JsValue::is_null(&source) => {
            js_sys::JsString::from("null")
        }
        source if JsValue::is_string(&source) => {
            let val: String = js_sys::JSON::stringify(&source).unwrap().into();
            js_sys::JsString::from(val.as_str())
        }
        source if JsValue::is_bigint(&source) => {
            let val = source.unchecked_into_f64().to_string();
            js_sys::JsString::from(val.as_str().to_owned() + "n")
        }
        source if JsValue::is_undefined(&source) => js_sys::JsString::from("undefined"),
        source if JsValue::js_typeof(&source) == "boolean" => {
            let val: &str = if *source == JsValue::TRUE {
                "true"
            } else {
                "false"
            };

            js_sys::JsString::from(val)
        }
        source if js_sys::Number::is_nan(&source) => js_sys::JsString::from("NaN"),
        source if JsValue::js_typeof(&source) == "number" && !js_sys::is_finite(&source) => {
            let val: &str = if source.unchecked_into_f64() > 0.0 {
                "Infinity"
            } else {
                "-Infinity"
            };

            js_sys::JsString::from(val)
        }
        source if JsValue::js_typeof(&source) == "number" => {
            let val = source.unchecked_into_f64().to_string();

            js_sys::JsString::from(val.as_str())
        }
        source if JsValue::is_array(&source) => {
            let js_array: Vec<JsValue> = js_sys::Array::from(source).to_vec();
            let mut result: String = String::from("");

            for val in js_array.iter() {
                result += ",";
                result += &stringify(&val).as_string().unwrap();
            }

            let array_start_token = "[".to_owned();
            let array_start_index = if result.len() > 0 { 1 } else { 0 };

            js_sys::JsString::from(array_start_token + &result[array_start_index..result.len()] + "]")
        }
        source if JsValue::is_object(&source) => {
            let js_obj: js_sys::Object = source.as_ref().clone().unchecked_into();
            let js_obj_keys: Vec<JsValue> = js_sys::Object::keys(&js_obj).to_vec();
            let mut result: Vec<String> = vec![];
            for key in js_obj_keys.iter() {
                let obj_val = js_sys::Reflect::get(&js_obj, key).unwrap();

                let value: String = (&key).as_string().unwrap().to_owned()
                    + ":"
                    + stringify(&obj_val).as_string().unwrap().to_owned().as_str();

                result.push(value);
            }

            let val: String = result.join(",");
            let object_start_token = "{".to_owned();
            js_sys::JsString::from(object_start_token + &val + "}")
        }
        _ => js_sys::JSON::stringify(&source).unwrap(),
    };
}
