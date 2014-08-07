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

#![feature(phase)]

extern crate serialize;

extern crate docopt;

#[phase(plugin)] extern crate docopt_macros;
#[phase(link, plugin)] extern crate log;


docopt!(Args, "
Usage: protogen [-l] [-o <outfile>] <infile>

Options:
    -o, --output    File to write generated code to.
    -l, --lex-only  Only run the lexer.
")

mod parser;

fn main() {
    use docopt::FlagParser;

    let args: Args = FlagParser::parse().unwrap_or_else(|e| e.exit());

    println!("Args: {}", args);

    if args.flag_lex_only {
        parser::lex(args.arg_infile);
        return;
    }
    let proto = parser::parse(args.arg_infile);

    println!("{}", proto);
}
