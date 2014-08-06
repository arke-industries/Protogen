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
    reader: std::iter::Peekable<char, R>,
}

impl<R: Iterator<char>> Iterator<Token> for Lexer<R> {
    fn next(&mut self) -> Option<Token> {
        // IO errors are interesting and never expected.
        let mut ident_token = None;
        for c in self.reader {
            // could be anything...
            match c {
                // string literal
                '"' => {
                    return Some(STRING(self.reader.by_ref().take_while(|&c| c != '"').collect()));
                }
                '\n' => {
                    return Some(NL);
                },
                '\r' => {
                    if self.reader.next().unwrap() != '\n' {
                        error!("CR found not followed by LF!");
                    }
                    return Some(NL);
                }
                // keyword or identifier
                c @ 'a'..'z' | c @ 'A'..'Z' | c @ '_' => {
                    let mut ident = String::with_capacity(16);
                    ident.push_char(c);
                    loop {
                        let c = *self.reader.peek().unwrap();
                        if c.is_alphanumeric() || c == '_' {
                            let c = self.reader.next().unwrap();
                            ident.push_char(c);
                        } else {
                            break;
                        }
                    }

                    // we just accidentally read past the character after the ident. do some
                    // shenanigans to get outside of the chrs borrow and seek.
                    ident_token = match ident.as_slice() {
                        "auth" => Some(ATTR(Auth)),
                        "unauth" => Some(ATTR(Unauth)),
                        "admin" => Some(ATTR(Admin)),
                        "global" => Some(ATTR(Global)),
                        "map" => Some(ATTR(Map)),
                        "newtype" => Some(NEWTYPE),
                        "category" => Some(CATEGORY),
                        "include" => Some(INCLUDE),
                        "method" => Some(METHOD),
                        "in" => Some(IN),
                        "out" => Some(OUT),
                        non_keyword => {
                            match try_parse_type(non_keyword) {
                                Some(t) => Some(PRIM(t)),
                                None => Some(IDENT(non_keyword.to_string()))
                            }
                        }
                    };
                    break
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
        ident_token
    }
}

pub fn parse<R: Iterator<char>>(f: std::iter::Peekable<char, R>) -> Result<Protocol, ParseError> {
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

