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

#[deriving(Show, Clone, Ord, Eq, PartialOrd, PartialEq, Hash, Encodable, Decodable)]
pub enum Type {
    I8, U8,
    I16, U16,
    I32, U32, F32,
    I64, U64, F64,
    Array(Box<Type>),
    Aggregate(Object),
    NamedType(String),
}

#[deriving(Show, Clone, Ord, Eq, PartialOrd, PartialEq, Hash, Encodable, Decodable)]
pub struct Object {
    fields: Vec<(String, Type)>,
}

#[deriving(Show)]
pub struct Category {
    id: u64,
    methods: HashMap<String, Method>,
}

#[deriving(Show, Encodable, Decodable)]
pub struct Method {
    comment: String,
    id: u64,
    properties: HashMap<String, Type>,
    attributes: Vec<Attr>,
}

#[deriving(Show, Ord, Eq, PartialOrd, PartialEq, Hash, Encodable, Decodable)]
pub enum ParseError {
    UnexpectedToken(String),
    UnexpectedEof(String),
    Other(String)
}

#[deriving(Show, Clone, Ord, Eq, PartialOrd, PartialEq, Hash, Encodable, Decodable)]
pub enum Attr {
    Auth,
    Unauth,
    Admin,
    Global,
    Map,
}

#[deriving(Show, Clone, Ord, Eq, PartialOrd, PartialEq, Hash, Encodable, Decodable)]
enum Token {
    ATTR(Attr),
    STRING(String),
    IDENT(String),
    PRIM(Type),
    COMMENT(String),
    LIT(u64),

    NEWTYPE,
    CATEGORY,
    INCLUDE,
    METHOD,

    SEMI,
    LBRACE,
    RBRACE,
    COMMA,
    EQ,
    COLON,

    EOF,
}

struct Lexer<R> {
    reader: std::iter::Peekable<char, R>,
    done: bool,
}

type BufferedReader = std::io::BufferedReader<Result<std::io::fs::File, std::io::IoError>>;
type Filator<'a, 'b> = std::iter::Map<'a,
    Result<char, std::io::IoError>,
    char,
    std::io::Chars<'b, BufferedReader>
>;

impl<'a> Lexer<Filator<'static, 'a>> {
    fn new(buf: &'a mut BufferedReader) -> Lexer<Filator<'static, 'a>> {
        Lexer {
            done: false,
            reader: buf.chars().map(|io| io.unwrap()).peekable(),
        }
    }
}

impl<R: Iterator<char>> Iterator<Token> for Lexer<R> {
    fn next(&mut self) -> Option<Token> {
        if self.done {
            return None;
        }
        // IO errors are interesting and never expected.
        let mut ident_token = None;
        for c in self.reader {
            // could be anything...
            match c {
                // string literal
                '"' => {
                    return Some(STRING(self.reader.by_ref().take_while(|&c| c != '"').collect()));
                }
                '\r' => {
                    if *self.reader.peek().unwrap() != '\n' {
                        error!("CR found not followed by LF!");
                    } else {
                        self.reader.next();
                    }
                },
                '\'' => {
                    let c: String = self.reader.by_ref().take_while(|&c| c != '\n' && c != '\r').collect();
                    let mut c = c.as_slice();
                    if c.ends_with("\r") {
                        if self.reader.next().unwrap() != '\n' {
                            error!("CR found not follwed by LF!");
                        }
                        c = c.slice_to(c.len() - 2)
                    }
                    return Some(COMMENT(c.to_string()));
                },
                c @ '0'..'9' => {
                    let mut lit = String::new();
                    lit.push_char(c);
                    loop {
                        if self.reader.peek().unwrap().is_digit() {
                            let c = self.reader.next().unwrap();
                            lit.push_char(c);
                        } else {
                            break;
                        }
                    }
                    return Some(LIT(from_str(lit.as_slice()).expect("Lexer accepted invalid numeric literal!")));
                },
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
                ' ' | '\t' | '\n' => continue,
                wat => error!("Found unexpected character when lexing: `{}`", wat),
            }
        }
        match ident_token {
            Some(t) => Some(t),
            None => { self.done = true; Some(EOF) }
        }
    }
}

struct TokenRepr {
    tag: u8,
    // ... fields ...
}

fn token_types_eq(t1: &Token, t2: &Token) -> bool {
    use std::mem::transmute;
    let (a, b): (&TokenRepr, &TokenRepr) = unsafe { (transmute(t1), transmute(t2)) };
    a.tag == b.tag
}

struct Parser<R> {
    lexer: Lexer<R>,
    lookahead: Token,
}

impl<R: Iterator<char>> Parser<R> {
    fn new(mut l: Lexer<R>) -> Parser<R> {
        let next = l.next().unwrap();
        Parser {
            lexer: l,
            lookahead: next
        }
    }

    fn next(&mut self) -> Token {
        // if we're at EOF already, the lexer is going to return None. This is harmless, fill it
        // with EOF instead.
        let mut next = self.lexer.next().unwrap_or(EOF);
        std::mem::swap(&mut self.lookahead, &mut next);
        debug!("next: got token {}, lookahead is {}", next, self.lookahead);
        next
    }

    fn expect(&mut self, tok: Token) -> Token {
        let next = self.next();
        if !token_types_eq(&tok, &next) {
            debug!("expect lookahead: {}", self.lookahead);
            fail!("Expected `{}`, found `{}`", tok, next);
        }
        next
    }

    fn expect_one_of(&mut self, toks: &[Token]) -> Token {
        let next = self.lookahead.clone();
        debug!("expect_one_of lookahead: {}", next);
        for tok in toks.iter() {
            if token_types_eq(tok, &next) {
                return self.next();
            }
        }
        fail!("Unexpected token `{}`, expected one of: {}", next, toks.iter().map(|t| t.to_string()).collect::<Vec<String>>().connect(", "))
    }

    fn expect_string(&mut self) -> String {
        match self.expect(STRING(String::new())) {
            STRING(s) => s,
            _ => unreachable!()
        }
    }

    fn expect_ident(&mut self) -> String {
        match self.expect(IDENT(String::new())) {
            IDENT(s) => s,
            _ => unreachable!()
        }
    }

    fn expect_lit(&mut self) -> u64 {
        match self.expect(LIT(0)) {
            LIT(lit) => lit,
            _ => unreachable!()
        }
    }

    fn parse_protocol(&mut self) -> Protocol {
        let mut proto = Protocol {
            types: HashMap::new(),
            categories: HashMap::new()
        };

        loop {
            match self.expect_one_of([NEWTYPE, CATEGORY, INCLUDE, EOF]) {
                NEWTYPE => {
                    let (name, type_) = self.parse_newtype();
                    proto.types.insert(name, type_);
                },
                CATEGORY => {
                    let (name, category) = self.parse_category();
                    proto.categories.insert(name, category);
                },
                INCLUDE => {
                    let p = self.expect_string();
                    let mut buf = open(p);
                    let mut parser = Parser::new(Lexer::new(&mut buf));
                    let Protocol { types, categories } = parser.parse_protocol();
                    proto.types.extend(types.move_iter());
                    proto.categories.extend(categories.move_iter());
                    self.expect(SEMI);
                },
                EOF => break,
                _ => unreachable!(),
            }
        }

        proto
    }

    fn parse_newtype(&mut self) -> (String, Type) {
        let name = self.expect_ident();
        self.expect(EQ);
        let ty = self.parse_type();
        match ty {
            Aggregate(..) => { },
            _ => { self.expect(SEMI); }
        }
        (name, ty)
    }

    fn parse_category(&mut self) -> (String, Category) {
        let name = self.expect_ident();
        self.expect(EQ);
        let id = self.expect_lit();
        self.expect(LBRACE);

        let mut cat = Category { id: id, methods: HashMap::new() };
        self.parse_category_body(&mut cat);
        (name, cat)
    }

    fn parse_category_body(&mut self, cat: &mut Category) {
        loop {
            match self.expect_one_of([INCLUDE, METHOD, RBRACE, EOF]) {
                EOF | RBRACE => break,
                INCLUDE => {
                    let s = self.expect_string();
                    let mut buf = open(s);
                    let mut parser = Parser::new(Lexer::new(&mut buf));
                    parser.parse_category_body(cat);
                    self.expect(SEMI);
                },
                METHOD => {
                    let name = self.expect_ident();
                    let meth = self.parse_method();
                    cat.methods.insert(name, meth);
                },
                _ => unreachable!()
            }
        }
    }

    fn parse_method(&mut self) -> Method {
        self.expect(EQ);
        let id = self.expect_lit();
        self.expect(LBRACE);
        let mut attrs = Vec::new();
        loop {
            match self.expect_one_of([ATTR(Auth), RBRACE]) {
                ATTR(a) => {
                    attrs.push(a);
                    match self.lookahead {
                        COMMA => { debug_assert_eq!(COMMA, self.next()); },
                        _ => { }
                    }
                },
                RBRACE => break,
                _ => unreachable!(),
            }
        }
        let mut comment = String::new();
        let mut props = HashMap::new();
        let mut first_prop = None;
        self.expect(LBRACE);
        loop {
            match self.expect_one_of([COMMENT(String::new()), IDENT(String::new()), RBRACE]) {
                // slice off the leading '
                COMMENT(c) => { comment.push_str(c.as_slice().slice_from(1)); comment.push_char('\n'); },
                IDENT(c) => { first_prop = Some(c); break },
                RBRACE => break,
                _ => unreachable!(),
            }
        }
        match first_prop {
            Some(id) => {
                props.insert(id, self.parse_property());
                loop {
                    match self.expect_one_of([IDENT(String::new()), RBRACE]) {
                        IDENT(i) => { props.insert(i, self.parse_property()); },
                        RBRACE => break,
                        _ => unreachable!()
                    }
                }
            },
            None => { }
        }

        Method {
            id: id,
            comment: comment,
            properties: props,
            attributes: attrs,
        }
    }

    fn parse_property(&mut self) -> Type {
        self.expect(EQ);
        let ty = self.parse_type();
        self.expect(SEMI);
        ty
    }

    fn parse_type(&mut self) -> Type {
        match self.expect_one_of([PRIM(I8), IDENT(String::new()), LBRACE]) {
            PRIM(ty) => ty,
            IDENT(ty) => NamedType(ty),
            LBRACE => {
                self.parse_aggregate()
            },
            _ => unreachable!(),
        }
    }

    fn parse_aggregate(&mut self) -> Type {
        let mut fields = Vec::new();
        loop {
            match self.expect_one_of([IDENT(String::new()), RBRACE, COMMA]) {
                IDENT(name) => {
                    self.expect(COLON);
                    let type_ = self.parse_type();
                    fields.push((name, type_));
                    match self.lookahead {
                        COMMA => { debug_assert_eq!(COMMA, self.next()); },
                        _ => { }
                    }
                },
                COMMA => continue,
                RBRACE => break,
                _ => unreachable!(),
            }
        }
        Aggregate(Object { fields: fields })
    }
}

pub fn parse(file: String) -> Result<Protocol, ParseError> {
    let mut buf = open(file);
    let mut parser = Parser::new(Lexer::new(&mut buf));

    Ok(parser.parse_protocol())
}

pub fn lex(file: String) {
    let mut buf = open(file);
    let mut lexer = Lexer::new(&mut buf);
    for tok in lexer {
        println!("{}", tok);
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
            Some(Array(box try_parse_type(other.slice(6, other.len() - 2)).unwrap()))
        }
        _ => {
            None
        }
    }
}

fn open(p: String) -> BufferedReader {
    std::io::BufferedReader::new(std::io::File::open(&Path::new(p.as_slice())))
}
