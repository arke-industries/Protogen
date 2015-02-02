Protogen
========

Protogen is used to generate a serialization/deserialization interface for the
Arke Industries Interchange Protocol (AIIP). It is made available under the
terms of the [MIT license](LICENSE).

Overview
--------

A protocol consists of RPC methods and objects. Every procedure has an ID
which consists of a method and a category. A procedure takes a number of
objects and returns a number of objects. What they are and how many is
determined by the method signature.

An object is either a primitive type, or an ordered collection of objects
(each of which is called a field).  Objects have no means to identify
themselves or their fields from inspecting their own representation. The
Protocol defines what objects are used in what contexts.

protogen's serialization/deserialization model depends on a "stream" which
provides the direct ability to read or write primitive types. The primitive
types protogen requires are:

- `array`
- `{u,i}{8, 16, 24, 32, 64}`
- `f{32,64}`
- `string`

`array` is an "unsized type". It consists of `{N, val1, val2, ...valN}`, where
`N` is the amount of elements (as a `u16`). They are homogeneous. Strings are
similar. More details later.

Text Format
-----------

An example showing every feature of protogen is available in `example.pg`.

`include` is literal, textual inclusion, along the lines of the C preprocessor
(though without any of its other features).

There is a grammar in `grammar.ebnf`


Object Layout
-------------

protogen is primarily concerned with shuttling data across the wire, and as
such does not add any padding to objects, and does not require any alignment.
Each field of an object is laid out one after another, using only the exact
amount of bytes required to store the field.

Running Protogen
----------------

To run `protogen`, install Python and pip, run `pip install -r
requirements.txt`, `make`, and then `./protogen path/to/definition.pg`.
