grammar Protogen;

ATTR : 'auth' | 'unauth' | 'admin' | 'global' | 'map' ;
NEWTYPE : 'newtype' ;
CATEGORY : 'category' ;
INCLUDE : 'include' ;
METHOD : 'method' ;
PRIM
    : 'i8' | 'u8'
    | 'i16' | 'u16'
    | 'i32' | 'u32' | 'f32'
    | 'i64' | 'u64' | 'f64'
    | 'array' '<' IDENT '>'
    ;
SEMI : ';' ;
LBRACE : '{' ;
RBRACE : '}' ;
COMMA : ',' ;
EQ : '=' ;
IN : 'in' ;
OUT : 'out' ;

STRING : '"' ~["]* '"' ;
IDENT : [a-zA-Z_][a-zA-Z_0-9]* ;
WS : [ \t]+ -> skip;
NL : '\n' | '\r\n' ;
EXTRA : . ;

newtype : NEWTYPE IDENT EQ (PRIM SEMI | object) ;
category : CATEGORY IDENT LBRACE NL? (include | method)* RBRACE NL? ;
include : INCLUDE STRING SEMI NL? ;
property : (IN | OUT) '=' (object | PRIM) SEMI NL ?;
comment : .*? NL ;
method : METHOD IDENT LBRACE ATTR* RBRACE LBRACE NL? comment* property* RBRACE NL ?;
object : LBRACE NL? (field COMMA NL?)* (field COMMA? NL?)? NL? RBRACE ;
field : IDENT ':' IDENT ;

prog : (newtype | category | NL)* ;
