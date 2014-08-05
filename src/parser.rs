// Copyright (c) 2014 Corey Richardson
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

use std::hashmap::HashMap;

pub struct Protocol {
    types: HashMap<String, Type>,
    categories: HashMap<String, Category>,
}

pub enum Type {
    I8, U8,
    I16, U16,
    I32, U32, F32,
    I64, U64, F64,
    Array(Type),
    Aggregate(Object),
}

pub struct Object {
    fields: Vec<(String, Type)>,
}

pub struct Category {
    name: String,
    methods: HashMap<String, Method>,
}

pub struct Method {
    comment: String,
    properties: HashMap<String, Property>,
    attributes: Vec<Attr>,
}

pub enum Property {
    In(Type),
    Out(Type)
}

pub enum ParseError {
    UnexpectedToken(String),
    UnexpectedEof(String),
    IoError(std::io::IoError),
    Other(String)
}

pub enum Attr {
    Auth,
    Unauth,
    Admin,
    Global,
    Map,
}

enum Token {
    ATTR(Attr),
    NEWTYPE,
    CATEGORY,
    INCLUDE,
    METHOD,
    PRIM(Type),
    SEMI,
    LBRACE,
    RBRACE,
    COMMA,
    EQ,
    IN,
    OUT,
    STRING(String),
    IDENT(String),
    NL,
}

struct Lexer<R> {
    reader: R,
}

impl<R: std::io::Buffer> Iterator<Token> for Lexer<R> {
    fn next(&mut self) -> Token {
        // IO errors are interesting and never expected.
        let mut chrs = self.reader.chars().map(|io| io.unwrap()).peekable();
        for c in chrs {
            // could be anything...
            match c {
                // string literal
                '"' => {
                    return STRING(chrs.take_while(|c| c != '"').collect());
                }
                '\n' => {
                    return NL;
                },
                '\r' => {
                    if chrs.next().unwrap() != '\n' {
                        error!("CR found not followed by LF!");
                    }
                    return NL;
                }
                // keyword or identifier
                'a'..'z' | 'A'..'Z' | '_' => {
                    let ident = chrs.take_while(|c| !c.is_whitespace()).collect::<String>();
                    match ident.as_slice() {
                        "auth" => return Some(ATTR(Auth)),
                        "unauth" => return Some(ATTR(Unauth)),
                        "admin" => return Some(ATTR(Admin)),
                        "global" => return Some(ATTR(Global)),
                        "map" => return Some(ATTR(Map)),
                        "newtype" => return Some(NEWTYPE),
                        "category" => return Some(CATEGORY),
                        "include" => return Some(INCLUDE),
                        "method" => return Some(METHOD),
                        non_keyword => {
                            match try_parse_type(non_keyword) {
                                Some(t) => return Some(PRIM(T)),
                                None => return Some(IDENT(non_keyword.to_string()))
                            }
                        }
                    }
                },
                ' ' | '\t' => continue,
                wat => error!("Found unexpected character when lexing: {}", wat),
            }
        }
    }
}

pub fn parse<R: std::io::Buffer>(f: R) -> Result<Protocol, ParseError> {
    let mut proto = Protocol {
        types: HashMap::new(),
        categories: HashMap::new(),
    };

    loop {
        let c = f.read_char();
        match c {
            'n' => {
                match f.read_until(b' ') {

                }
            },
            'c' => {

            },
            _ => { }
        }
    }
}

fn try_parse_type(s: &str) -> Option<Type> {
    match s {
        "i8" => Some(I8),
        "u8" => Some(U8),
        "i16" => Some(I16),
        "u16" => Some(U16),
        "i32" => Some(I32),
        "u32" => Some(U32),
        "f32" => Some(F32),
        "i64" => Some(I64),
        "u64" => Some(U64),
        "f64" => Some(F64),
        other if other.starts_with("array<") => {
            Some(Array(try_parse_type(other.slice(6, other.len() - 2)).unwrap()));
        }
        _ => {
            None
        }
}

