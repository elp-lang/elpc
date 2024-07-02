%start program
%token IMPORT FROM EXPORT INTERFACE ENUM COMPONENT FN VAR FORM EMAILINPUT SECURETEXTINPUT BUTTON TEXT OPTIONAL PROTECTED STATEFUL NIL STRING INT LT GT LBRACE RBRACE LPAREN RPAREN COMMA EQUAL DOT ARROW NOTNIL TRUE FALSE IDENTIFIER ANNOTATION STRING_LITERAL NUMBER
%%

program: imports interfaces enums components;

imports: import_stmt*;
import_stmt: IMPORT IDENTIFIER FROM STRING_LITERAL;

interfaces: interface_stmt*;
interface_stmt: INTERFACE IDENTIFIER LBRACE interface_body RBRACE;
interface_body: (interface_prop SEMI)*;
interface_prop: IDENTIFIER type;

type: IDENTIFIER
    | OPTIONAL LT IDENTIFIER GT
    | PROTECTED LT IDENTIFIER GT;

enums: enum_stmt*;
enum_stmt: ANNOTATION ENUM IDENTIFIER LBRACE enum_body RBRACE;
enum_body: (enum_variant SEMI)*;
enum_variant: IDENTIFIER
            | IDENTIFIER LPAREN IDENTIFIER RPAREN;

components: component_stmt*;
component_stmt: COMPONENT IDENTIFIER LT IDENTIFIER COMMA IDENTIFIER GT LBRACE component_body RBRACE;
component_body: (component_export | component_var | component_fn | component_form)*;

component_export: EXPORT IDENTIFIER STATEFUL LT type GT;
component_var: VAR IDENTIFIER EQUAL expr;
component_fn: FN IDENTIFIER LPAREN params RPAREN block;
component_form: FORM LPAREN onSubmit RPAREN LBRACE form_body RBRACE;

onSubmit: IDENTIFIER EQUAL IDENTIFIER;
form_body: (form_element)*;
form_element: EMAILINPUT LPAREN ref COMMA placeholder RPAREN
             | SECURETEXTINPUT LPAREN ref COMMA placeholder RPAREN
             | BUTTON LBRACE TEXT LPAREN STRING_LITERAL RPAREN RBRACE;

ref: "&" IDENTIFIER;
placeholder: "placeholder" EQUAL STRING_LITERAL;

block: LBRACE statements RBRACE;
statements: statement*;
statement: expr SEMI
         | VAR IDENTIFIER EQUAL expr SEMI
         | IF expr block
         | THROW IDENTIFIER SEMI;

expr: IDENTIFIER
    | IDENTIFIER DOT IDENTIFIER
    | IDENTIFIER LPAREN args RPAREN
    | STRING_LITERAL
    | NUMBER
    | NOTNIL;

params: (param (COMMA param)*)?;
param: IDENTIFIER type;

args: (arg (COMMA arg)*)?;
arg: IDENTIFIER;

%%

