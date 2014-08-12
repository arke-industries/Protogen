grammar Protogen;

NEWTYPE : 'newtype' ;
CATEGORY : 'category' ;
INCLUDE : 'include' ;
METHOD : 'method' ;
ARRAY : 'array' ;
MAP : 'map' ;
PRIM
    : 'i8' | 'u8'
    | 'i16' | 'u16'
    | 'i32' | 'u32' | 'f32'
    | 'i64' | 'u64' | 'f64'
    ;
SEMI : ';' ;
LBRACE : '{' ;
RBRACE : '}' ;
LT : '<' ;
GT : '>' ;
COMMA : ',' ;
EQ : '=' ;

STRING : '"' ~["]* '"' ;
IDENT : [a-zA-Z_][a-zA-Z_0-9]* ;
LIT : [0-9]+ ;
WS : [ \r\n\t]+ -> skip;
COMMENT : '\'' .*? ('\r\n' | '\n') ;

type : PRIM | ARRAY LT type GT | MAP LT type COMMA type GT | IDENT | object;
newtype : NEWTYPE IDENT EQ type  SEMI ;
category : CATEGORY IDENT EQ LIT LBRACE (include | method)* RBRACE ;
include : INCLUDE STRING SEMI ;
property : IDENT EQ object SEMI ;
method : METHOD IDENT EQ LIT LBRACE IDENT* RBRACE LBRACE COMMENT* property* RBRACE ;
object : LBRACE (field COMMA)* (field COMMA?)? RBRACE ;
field : IDENT ':' type ;

protocol : (newtype | category | include)* ;
