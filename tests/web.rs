use wasm_bindgen_test::wasm_bindgen_test_configure;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;
use js_sys;

use eson::parse::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn eson_parse_browser() {
    let source_code = "{
        name: 13,
        value: 'baz',
        isDefined: false,
        isNotDefined: true
        // This is a response from some API
        data: undefined,
        valid_until: NaN,
        valid_from: 1763225669356
        /*
            All cool guys use buffer like this:
            [1, 2, 3] // Yes, it is a comment inside another one!
        */
        buffer: [16, 21, 51, 0, 0, 0, 0],
        src: ['a', 'b', 'c'],
        source: 'abc'
    }";
    
    let string = js_sys::JsString::from(source_code);
    let res = parse(string);
    let result_string = js_sys::Reflect::get(&res, &js_sys::JsString::from("value")).unwrap();
    let result_number: JsValue = js_sys::Reflect::get(&res, &js_sys::JsString::from("name")).unwrap();
    let expected_string: JsValue = js_sys::JsString::from("baz").into();
    let expected_number: JsValue = js_sys::Number::from(13).into();

    assert_eq!(result_string, expected_string);
    assert_eq!(result_number, expected_number);
}

#[wasm_bindgen_test]
fn eson_stringify_browser() {
    use eson::stringify::*;

    let obj = js_sys::Object::new().into();

    // String
    js_sys::Reflect::set(&obj, &js_sys::JsString::from("str"), &js_sys::JsString::from("hello")).unwrap();

    // String with escaped quotes
    js_sys::Reflect::set(&obj, &js_sys::JsString::from("str_escaped"), &js_sys::JsString::from("hello\"world'test")).unwrap();

    // String with newlines
    js_sys::Reflect::set(&obj, &js_sys::JsString::from("str_multiline"), &js_sys::JsString::from("line1\nline2\nline3")).unwrap();

    // Number (positive)
    js_sys::Reflect::set(&obj, &js_sys::JsString::from("num_42"), &js_sys::Number::from(42)).unwrap();

    // Number (negative)
    js_sys::Reflect::set(&obj, &js_sys::JsString::from("num_negative"), &js_sys::Number::from(-42)).unwrap();

    // Number (float)
    js_sys::Reflect::set(&obj, &js_sys::JsString::from("num_float"), &js_sys::Number::from(3.14159)).unwrap();

    // Number (negative float)
    js_sys::Reflect::set(&obj, &js_sys::JsString::from("num_negative_float"), &js_sys::Number::from(-3.14159)).unwrap();

    // Boolean true
    js_sys::Reflect::set(&obj, &js_sys::JsString::from("bool_true"), &JsValue::from(true)).unwrap();

    // Boolean false
    js_sys::Reflect::set(&obj, &js_sys::JsString::from("bool_false"), &JsValue::from(false)).unwrap();

    // Null
    js_sys::Reflect::set(&obj, &js_sys::JsString::from("null_val"), &JsValue::null()).unwrap();

    // NaN
    js_sys::Reflect::set(&obj, &js_sys::JsString::from("nan_val"), &JsValue::from(f64::NAN)).unwrap();

    // Infinity (positive)
    let val: JsValue = js_sys::Number::POSITIVE_INFINITY.into();
    js_sys::Reflect::set(&obj, &js_sys::JsString::from("infinity_val"), &val).unwrap();

    // Infinity (negative)
    let neg_infinity: JsValue = js_sys::Number::NEGATIVE_INFINITY.into();
    js_sys::Reflect::set(&obj, &js_sys::JsString::from("infinity_negative"), &neg_infinity).unwrap();

    // BigInt (positive)
    let bigint_pos = JsValue::bigint_from_str("123456789");
    js_sys::Reflect::set(&obj, &js_sys::JsString::from("bigint_pos"), &bigint_pos).unwrap();

    // BigInt (negative)
    let bigint_neg = JsValue::bigint_from_str("-987654321");
    js_sys::Reflect::set(&obj, &js_sys::JsString::from("bigint_neg"), &bigint_neg).unwrap();

    // Array with mixed types
    let arr = js_sys::Array::new();
    arr.push(&js_sys::Number::from(1));
    arr.push(&js_sys::JsString::from("two"));
    arr.push(&JsValue::from(true));
    arr.push(&js_sys::Number::from(-5.5));
    arr.push(&JsValue::null());
    let bigint_in_arr = JsValue::bigint_from_str("999");
    arr.push(&bigint_in_arr);
    js_sys::Reflect::set(&obj, &js_sys::JsString::from("arr"), &arr).unwrap();

    // Nested Object
    let nested = js_sys::Object::new().into();
    js_sys::Reflect::set(&nested, &js_sys::JsString::from("inner"), &js_sys::JsString::from("value")).unwrap();
    js_sys::Reflect::set(&nested, &js_sys::JsString::from("nested_num"), &js_sys::Number::from(-99.99)).unwrap();
    
    // Empty array in nested object
    let empty_arr = js_sys::Array::new();
    js_sys::Reflect::set(&nested, &js_sys::JsString::from("empty_arr"), &empty_arr).unwrap();

    // Empty object in nested object
    let empty_obj = js_sys::Object::new().into();
    js_sys::Reflect::set(&nested, &js_sys::JsString::from("empty_obj"), &empty_obj).unwrap();

    // Deeply nested array [[[0]]]
    let inner_arr = js_sys::Array::new();
    inner_arr.push(&js_sys::Number::from(0));
    let middle_arr = js_sys::Array::new();
    middle_arr.push(&inner_arr);
    let outer_arr = js_sys::Array::new();
    outer_arr.push(&middle_arr);
    js_sys::Reflect::set(&nested, &js_sys::JsString::from("deep_array"), &outer_arr).unwrap();
    
    js_sys::Reflect::set(&obj, &js_sys::JsString::from("nested"), &nested).unwrap();

    let res = stringify(&obj);

    // Verify the stringified output contains expected patterns
    let res_str = res.as_string().unwrap();
    assert!(res_str.contains("str:\"hello\""));
    assert!(res_str.contains("str_escaped:\"hello\\\"world'test\""));
    assert!(res_str.contains("str_multiline:\"line1\\nline2\\nline3\""));
    assert!(res_str.contains("num_42:42"));
    assert!(res_str.contains("num_negative:-42"));
    assert!(res_str.contains("num_float:3.14159"));
    assert!(res_str.contains("num_negative_float:-3.14159"));
    assert!(res_str.contains("bool_true:true"));
    assert!(res_str.contains("bool_false:false"));
    assert!(res_str.contains("null_val:null"));
    assert!(res_str.contains("nan_val:NaN"));
    assert!(res_str.contains("infinity_val:Infinity"));
    assert!(res_str.contains("infinity_negative:-Infinity"));
    assert!(res_str.contains("bigint_pos:123456789n"));
    assert!(res_str.contains("bigint_neg:-987654321n"));
    assert!(res_str.contains("arr:[1,\"two\",true,-5.5,null,999n]"));
    assert!(res_str.contains("nested:{inner:\"value\",nested_num:-99.99,empty_arr:[],empty_obj:{},deep_array:[[[0]]]}"));
}