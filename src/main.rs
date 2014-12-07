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

#![feature(phase, globs)]

extern crate serialize;

extern crate docopt;

#[phase(plugin)] extern crate docopt_macros;
#[phase(link, plugin)] extern crate log;


docopt!(Args deriving Show, "
Usage: protogen [-l | -p | -c] [-o <outfile>] <infile>

Options:
    -o, --output      File to write generated code to.
    -l, --lex-only    Only run the lexer, printing out the token stream.
    -p, --parse-only  Run up to the parser, printing out the parsed protocol.
    -c, --check-only  Run up to validity checking, printing out the analysis.
")

mod parser;
//mod check;

fn main() {
    let args: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());

    debug!("Args: {}", args);

    if args.flag_lex_only {
        parser::lex(args.arg_infile);
        return;
    }
    let proto = parser::parse(args.arg_infile).ok().expect("Failed to parse!");

    if args.flag_parse_only {
        println!("{}", proto);
        return;
    }

    //check::check(&proto);

    if args.flag_check_only {
        return;
    }
}
