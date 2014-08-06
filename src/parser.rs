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

use std::collections::hashmap::HashMap;
use std;

#[deriving(Show)]
pub struct Protocol {
    types: HashMap<String, Type>,
    categories: HashMap<String, Category>,
}

#[deriving(Show, Ord, Eq, PartialOrd, PartialEq, Hash, Encodable, Decodable)]
pub enum Type {
    I8, U8,
    I16, U16,
    I32, U32, F32,
    I64, U64, F64,
    Array(Box<Type>),
    Aggregate(Object),
}

#[deriving(Show, Ord, Eq, PartialOrd, PartialEq, Hash, Encodable, Decodable)]
pub struct Object {
    fields: Vec<(String, Type)>,
}

#[deriving(Show)]
pub struct Category {
    name: String,
    methods: HashMap<String, Method>,
}

#[deriving(Show, Encodable, Decodable)]
pub struct Method {
    comment: String,
    properties: HashMap<String, Property>,
    attributes: Vec<Attr>,
}

#[deriving(Show, Ord, Eq, PartialOrd, PartialEq, Hash, Encodable, Decodable)]
pub enum Property {
    In(Type),
    Out(Type)
}

#[deriving(Show, Ord, Eq, PartialOrd, PartialEq, Hash, Encodable, Decodable)]
pub enum ParseError {
    UnexpectedToken(String),
    UnexpectedEof(String),
    Other(String)
}

#[deriving(Show, Ord, Eq, PartialOrd, PartialEq, Hash, Encodable, Decodable)]
pub enum Attr {
    Auth,
    Unauth,
    Admin,
    Global,
    Map,
}

#[deriving(Show, Ord, Eq, PartialOrd, PartialEq, Hash, Encodable, Decodable)]
enum Token {
    ATTR(Attr),
    STRING(String),
    IDENT(String),
    PRIM(Type),

    NEWTYPE,
    CATEGORY,
    INCLUDE,
    METHOD,
    IN,
    OUT,

    SEMI,
    LBRACE,
    RBRACE,
    COMMA,
    EQ,
    COLON,

    NL,
}

struct Lexer<R> {
    reader: R,
}

impl<R: std::io::Buffer> Iterator<Token> for Lexer<R> {
    fn next(&mut self) -> Option<Token> {
        // IO errors are interesting and never expected.
        let mut chrs = self.reader.chars().map(|io| io.unwrap()).peekable();
        for c in chrs {
            // could be anything...
            match c {
                // string literal
                '"' => {
                    return Some(STRING(chrs.take_while(|&c| c != '"').collect()));
                }
                '\n' => {
                    return Some(NL);
                },
                '\r' => {
                    if chrs.next().unwrap() != '\n' {
                        error!("CR found not followed by LF!");
                    }
                    return Some(NL);
                }
                // keyword or identifier
                c @ 'a'..'z' | c @ 'A'..'Z' | c @ '_' => {
                    let ident = Some(c).move_iter().chain(chrs.take_while(|&c| c.is_alphanumeric() || c == '_')).collect::<String>();
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
                        "in" => return Some(IN),
                        "out" => return Some(OUT),
                        non_keyword => {
                            match try_parse_type(non_keyword) {
                                Some(t) => return Some(PRIM(t)),
                                None => return Some(IDENT(non_keyword.to_string()))
                            }
                        }
                    }
                },
                ';' => return Some(SEMI),
                ':' => return Some(COLON),
                '{' => return Some(LBRACE),
                '}' => return Some(RBRACE),
                ',' => return Some(COMMA),
                '=' => return Some(EQ),
                ' ' | '\t' => continue,
                wat => error!("Found unexpected character when lexing: {}", wat),
            }
        }
        None
    }
}

pub fn parse<R: std::io::Buffer>(f: R) -> Result<Protocol, ParseError> {
    let mut lex = Lexer { reader: f };
    for tok in lex {
        println!("{}", tok);
    }

    let mut proto = Protocol {
        types: HashMap::new(),
        categories: HashMap::new(),
    };

    Ok(proto)
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
            Some(Array(box try_parse_type(other.slice(6, other.len() - 2)).unwrap()))
        }
        _ => {
            None
        }
    }
}

