protogen
========

protogen is used to generate a serialization/deserialization interface for the
Arke Industries unified game protocol.

Overview
--------

A protocol consists of RPC procedures and objects. Every procedure has an ID
which consists of a method and a category. A procedure takes a number of
objects and returns a number of objects. What they are and how many is
determined by the procedure signature.

An object is either a primitive type, or an ordered collection of objects
(each of which is called a field).  Objects have no means to identify
themselves or their fields from inspecting their own representation. The
Protocol defines what objects are returned in what contexts.

protogen's serialization/deserialization model depends strongly on a "stream"
which provides the direct ability to read or write primitive types. The
primitive types protogen requires are:

- `array`
- `{u,i}{8, 16, 24, 32, 64}`
- `f{32,64}`

`array` is an "unsized type". It consists of `{N, val1, val2, ...valN}`, where
`N` is the amount of elements (as a `u16`). They are homogenous.

Stream
------

The stream is required to support serialization and deserialization of every
primitive, returning a value suitable for use in the host language.

Text Format
-----------

protogen's text format is rather simple. The exact grammary is NYI, but it
goes along the lines of:

```
newtype ObjectId = u64
newtype Coordinate = f64
newtype Size = u32

newtype MapObject = {
	x: Coordinate,
	y: Coordinate,
	planetId: ObjectId,
	width: Size,
	width: Size
}
category Map { include Map.pg }
category User {
	method Login {
		Documentation goes here.

		in = {email: string, password: string}
		out = {userId: ObjectId }
	}
}
```

`include` is literal, textual inclusion, along the lines of the C preprocessor
(though without any other of its features).

The in/out pair demonstrates anonymous object declaration. Everything in a
method is considered documentation or comments until the first line starting
with (sans whitespaces) `<ident> = ...`

Usage
-----

protogen generates structure definitions for all objects and
serialize/deserialize functions for every method's in/out pair. These take a
stream to write to/read from.
