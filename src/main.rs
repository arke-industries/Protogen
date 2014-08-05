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


use docopt::FlagParser;

docopt!(Args, "
Usage: protogen [-o <outfile>] <infile>

Options:
    -o, --output  File to write generated code to.
")

mod parser;

fn main() {
    let args: Args = FlagParser::parse().unwrap_or_else(|e| e.exit());

    println!("{}", args);
}
