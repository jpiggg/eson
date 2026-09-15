#![allow(
    dead_code
)]

use pest_derive;

#[derive(pest_derive::Parser)]
#[grammar = "src/parse/grammar/eson.pest"]
pub struct ESonParser;

pub mod ast {
    use super::Rule;
    use pest::Span;

    fn span_into_str(span: Span) -> &str {
        span.as_str()
    }

    fn extract_infinity(span: Span) -> &str {      
        let r = if span.as_str().starts_with("-") {
            "-1"
        } else {
            "1"
        };

        r
    }

    fn extract_key_into_str(span: Span) -> &str {
        let s = span.as_str();
        &s[1..s.len() - 1]
    }

    #[derive(PartialEq, Debug, pest_ast::FromPest)]
    #[pest_ast(rule(Rule::Boolean))]
    pub struct Boolean {
        #[pest_ast(outer(with(span_into_str), with(str::parse), with(Result::unwrap)))]
        pub value: bool
    }

    #[derive(PartialEq, Debug, pest_ast::FromPest)]
    #[pest_ast(rule(Rule::String))]
    pub struct String<'pest> {
        #[pest_ast(outer(with(span_into_str)))]
        pub value: &'pest str
    }
    

    #[derive(PartialEq, Debug, pest_ast::FromPest)]
    #[pest_ast(rule(Rule::Number))]
    pub struct Number {
        #[pest_ast(outer(with(span_into_str), with(str::parse), with(Result::unwrap)))]
        pub value: f64
    }

    #[derive(PartialEq, Debug, pest_ast::FromPest)]
    #[pest_ast(rule(Rule::Infinity))]
    pub struct Infinity {
        #[pest_ast(outer(with(extract_infinity), with(str::parse), with(Result::unwrap)))]
        pub value: i8,
    }

    #[derive(PartialEq, Debug, pest_ast::FromPest)]
    #[pest_ast(rule(Rule::undefined))]
    pub struct Undefined {}

    #[derive(PartialEq, Debug, pest_ast::FromPest)]
    #[pest_ast(rule(Rule::NaN))]
    pub struct NaN {}

    #[derive(PartialEq, Debug, pest_ast::FromPest)]
    #[pest_ast(rule(Rule::null))]
    pub struct Null {}

    #[derive(PartialEq, Debug, pest_ast::FromPest)]
    #[pest_ast(rule(Rule::BigInt))]
    pub struct BigInt<'pest> {
        #[pest_ast(outer(with(span_into_str)))]
        pub value: &'pest str
    }

    #[derive(PartialEq, Debug, pest_ast::FromPest)]
    #[pest_ast(rule(Rule::value))]
    pub enum Value<'pest> {
        Object(Box<Object<'pest>>),
        Array(Box<Array<'pest>>),
        String(String<'pest>),
        Number(Number),
        Boolean(Boolean),
        Null(Null),
        Undefined(Undefined),
        NaN(NaN),
        Infinity(Infinity),
        BigInt(BigInt<'pest>)
    }

    #[derive(PartialEq, Debug, pest_ast::FromPest)]
    #[pest_ast(rule(Rule::COMMENT))]
    pub struct Comment<'pest> {
        #[pest_ast(outer(with(span_into_str)))]
        pub value: &'pest str
    }

    #[derive(PartialEq, Debug, pest_ast::FromPest)]
    #[pest_ast(rule(Rule::key))]
    pub enum Key<'pest> {
        KeySimple(KeySimple<'pest>),
        KeyComplex(KeyComplex<'pest>),
    }

    #[derive(PartialEq, Debug, pest_ast::FromPest)]
    #[pest_ast(rule(Rule::key_simple))]
    pub struct KeySimple<'pest> {
        #[pest_ast(outer(with(span_into_str)))]
        pub value: &'pest str
    }
    
     #[derive(PartialEq, Debug, pest_ast::FromPest)]
    #[pest_ast(rule(Rule::key_complex))]
    pub struct KeyComplex<'pest> {
        #[pest_ast(outer(with(extract_key_into_str)))]
        pub value: &'pest str
    }

    #[derive(PartialEq, Debug, pest_ast::FromPest)]
    #[pest_ast(rule(Rule::pair))]
    pub struct Pair<'pest> {
        pub key: Key<'pest>,
        pub value: Value<'pest>,
        pub comment: Option<Comment<'pest>>
    }

    #[derive(PartialEq, Debug, pest_ast::FromPest)]
    #[pest_ast(rule(Rule::Array))]
    pub struct Array<'pest> {
        #[pest_ast(default(Vec::new()))]
        pub value: Vec<Value<'pest>>,
    }

    #[derive(PartialEq, Debug, pest_ast::FromPest)]
    #[pest_ast(rule(Rule::Object))]
    pub struct Object<'pest> {
        pub pair: Vec<Pair<'pest>>
    }


    #[derive(PartialEq, Debug, pest_ast::FromPest)]
    #[pest_ast(rule(Rule::ESon))]
    pub struct ESon<'pest> {
        pub object: Object<'pest>,
        pub _eoi: EOI,
    }

    #[derive(PartialEq, Debug, pest_ast::FromPest)]
    #[pest_ast(rule(Rule::EOI))]
    pub struct EOI;
}


#[cfg(test)]
mod tests {
    use from_pest::FromPest;
    use pest::Parser;
    use crate::parse::parser::ast;
    use pretty_assertions::{assert_eq};

    #[test]
    fn test_flat_object() {
        let source = String::from_utf8(std::fs::read("./examples/data.eson").unwrap()).unwrap();
        let mut parse_tree = crate::parse::parser::ESonParser::parse(crate::parse::parser::Rule::ESon, &source).unwrap();

        println!("----------> Parse tree: {:#?}", parse_tree);
        let syntax_tree: ast::ESon = ast::ESon::from_pest(&mut parse_tree).expect("infallible");

        let expected = ast::ESon {
            object: ast::Object {
                pair: vec![
                    ast::Pair {
                        key: ast::Key::KeySimple(ast::KeySimple {value: "str"}),
                        value: ast::Value::String(ast::String {value: "hello"}),
                        comment: None
                    },
                    ast::Pair {
                        key: ast::Key::KeyComplex(ast::KeyComplex {value: "str_escaped"}),
                        value: ast::Value::String(ast::String {value: "hello\\\"world'test"}),
                        comment: None
                    },
                    ast::Pair {
                        key: ast::Key::KeyComplex(ast::KeyComplex {value: "str_multiline"}),
                        value: ast::Value::String(ast::String {value: "line1\\nline2\\nline3"}),
                        comment: None
                    },
                    ast::Pair {
                        key: ast::Key::KeySimple(ast::KeySimple {value: "isDefined"}),
                        value: ast::Value::Undefined(ast::Undefined {}),
                        comment: None
                    },
                    ast::Pair {
                        key: ast::Key::KeyComplex(ast::KeyComplex {value: "num_42"}),
                        value: ast::Value::Number(ast::Number { value: 42.0 }),
                        comment: None
                    },
                    ast::Pair {
                        key: ast::Key::KeyComplex(ast::KeyComplex {value: "num_negative"}),
                        value: ast::Value::Number(ast::Number { value: -42.0 }),
                        comment: None
                    },
                    ast::Pair {
                        key: ast::Key::KeyComplex(ast::KeyComplex {value: "num_float"}),
                        value: ast::Value::Number(ast::Number { value: 3.14159 }),
                        comment: None
                    },
                    ast::Pair {
                        key: ast::Key::KeyComplex(ast::KeyComplex {value: "num_negative_float"}),
                        value: ast::Value::Number(ast::Number { value: -3.14159 }),
                        comment: None
                    },
                    ast::Pair {
                        key: ast::Key::KeyComplex(ast::KeyComplex {value: "bool_true"}),
                        value: ast::Value::Boolean(ast::Boolean { value: true }),
                        comment: None
                    },
                    ast::Pair {
                        key: ast::Key::KeyComplex(ast::KeyComplex {value: "bool_false"}),
                        value: ast::Value::Boolean(ast::Boolean { value: false }),
                        comment: None
                    },
                    ast::Pair {
                        key: ast::Key::KeyComplex(ast::KeyComplex {value: "null_val"}),
                        value: ast::Value::Null(ast::Null {}),
                        comment: Some(ast::Comment { value: "/*\n    All cool guys use buffer like this:\n    [1, 2, 3] // Yes, it is a comment inside another one!\n  */"})
                    },
                    ast::Pair {
                        key: ast::Key::KeyComplex(ast::KeyComplex {value: "nan_val"}),
                        value: ast::Value::NaN(ast::NaN {}),
                        comment: None
                    },
                    ast::Pair {
                        key: ast::Key::KeyComplex(ast::KeyComplex {value: "infinity_val"}),
                        value: ast::Value::Infinity(ast::Infinity {value: 1}),
                        comment: None
                    },
                    ast::Pair {
                        key: ast::Key::KeyComplex(ast::KeyComplex {value: "infinity_negative"}),
                        value: ast::Value::Infinity(ast::Infinity {value: -1}),
                        comment: None
                    },
                    ast::Pair {
                        key: ast::Key::KeyComplex(ast::KeyComplex {value: "bigint_pos"}),
                        value: ast::Value::BigInt(ast::BigInt { value: "123456789n" }),
                        comment: None
                    },
                    ast::Pair {
                        key: ast::Key::KeyComplex(ast::KeyComplex {value: "bigint_neg"}),
                        value: ast::Value::BigInt(ast::BigInt { value: "-987654321n" }),
                        comment: None
                    },
                    ast::Pair {
                        key: ast::Key::KeySimple(ast::KeySimple {value: "arr"}),
                        value: ast::Value::Array(Box::new(ast::Array {
                            value: vec![
                                ast::Value::Number(ast::Number { value: 1.0 }),
                                ast::Value::String(ast::String { value: "two" }),
                                ast::Value::Boolean(ast::Boolean { value: true }),
                                ast::Value::Number(ast::Number { value: -5.5 }),
                                ast::Value::Null(ast::Null {}),
                                ast::Value::BigInt(ast::BigInt { value: "999n" })
                            ]
                        })),
                        comment: None
                    },
                    ast::Pair {
                        key: ast::Key::KeySimple(ast::KeySimple {value: "nested"}),
                        value: ast::Value::Object(Box::new(ast::Object {
                            pair: vec![
                                ast::Pair {
                                    key: ast::Key::KeySimple(ast::KeySimple {value: "inner"}),
                                    value: ast::Value::String(ast::String {value: "value"}),
                                    comment: None
                                },
                                ast::Pair {
                                    key: ast::Key::KeyComplex(ast::KeyComplex {value: "nested_num"}),
                                    value: ast::Value::Number(ast::Number { value: -99.99 }),
                                    comment: None
                                },
                                ast::Pair {
                                    key: ast::Key::KeyComplex(ast::KeyComplex {value: "empty_arr"}),
                                    value: ast::Value::Array(Box::new(ast::Array { value: vec![] })),
                                    comment: None
                                },
                                ast::Pair {
                                    key: ast::Key::KeyComplex(ast::KeyComplex {value: "empty_obj"}),
                                    value: ast::Value::Object(Box::new(ast::Object { pair: vec![] })),
                                    comment: None
                                },
                                ast::Pair {
                                    key: ast::Key::KeyComplex(ast::KeyComplex {value: "deep_array"}),
                                    value: ast::Value::Array(Box::new(ast::Array {
                                        value: vec![
                                            ast::Value::Array(Box::new(ast::Array {
                                                value: vec![
                                                    ast::Value::Array(Box::new(ast::Array {
                                                        value: vec![
                                                            ast::Value::Number(ast::Number { value: 0.0 })
                                                        ]
                                                    }))
                                                ]
                                            }))
                                        ]
                                    })),
                                    comment: None
                                }
                            ]
                        })),
                        comment: None
                    }
                ]
            },
            _eoi: ast::EOI
        };

        assert_eq!(&syntax_tree, &expected);
    }
}