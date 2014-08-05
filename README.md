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
`N` is the amount of elements (as a `u16`). They are homogeneous.

Stream
------

The stream is required to support serialization and deserialization of every
primitive, returning a value suitable for use in the host language.

Text Format
-----------

protogen's text format is rather simple. The exact grammar is NYI, but it
goes along the lines of:

```
newtype ObjectId = u64;
newtype Coordinate = f64;
newtype Size = u32;

newtype MapObject = {
	x: Coordinate,
	y: Coordinate,
	planetId: ObjectId,
	width: Size,
	height: Size
}
category Map { include "Map.pg"; }
category User {
	method Login { unauth global } {
        Authenticates to the server using an email/password, returning the
        corresponding users' ID.

		in = {email: string, password: string};
		out = {userId: ObjectId };
	}
}
```

`include` is literal, textual inclusion, along the lines of the C preprocessor
(though without any of its other features).

The in/out pair demonstrates anonymous object declaration. Everything in a
method is considered documentation until the first line starting with (sans
whitespace) `<ident> = ...`

The attributes after the method name alter the permissions and availability of
that method. The `auth` attribute gives access to the method to any user that
has been authenticated. Similarly, the `unauth` attribute gives access to
unauthorized users. The `admin` attribute allows access only to users authorized
as an admin. The `global` and `map` attributes restrict the types of servers the
procedure can be called on. Map servers process any method related to the
physical map while global servers process any method not covered by a map
server. `admin` is mutually exclusive with both `auth` and `unauth`. `map` and
`global` are mutually exclusive. Any non-mutually-exclusive attributes may be
combined.

An approxmiate reference grammar, using antlr4, is provided in `Protogen.g4`.
Note that it is incorrect: due to the ambiguity between a comment and a
property, it considers `in = u32;` to be a comment.

Usage
-----

protogen generates structure definitions for all objects and
serialize/deserialize functions for every method's in/out pair. These take a
stream to write to/read from. It also generates a dispatcher function that
branches on the method type and category returning a new method handler.
