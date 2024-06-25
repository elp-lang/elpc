%start Program
%%

Program: ImportList DeclList
    ;

ImportList: Import* 
    ;

Import: "import" "{" ImportSpecList "}" "from" STRING
    ;

ImportSpecList: ImportSpec ("," ImportSpec)* 
    ;

ImportSpec: IDENT ( "as" IDENT )?
    ;

DeclList: Decl*
    ;

Decl: EnumDecl
    | ObjectDecl
    | FunctionDecl
    ;

EnumDecl: "export" "enum" IDENT "{" EnumVariantList "}"
    ;

EnumVariantList: EnumVariant ("," EnumVariant)* 
    ;

EnumVariant: "." IDENT "(" "Optional" "<" "string" ">" ")"
    ;

ObjectDecl: "export" "object" IDENT "implements" InterfaceList "{" FieldList "}"
    ;

InterfaceList: IDENT ("," IDENT)*
    ;

FieldList: Field*
    ;

Field: "." IDENT "=" Expr "`json:" STRING "`"
    | "." IDENT IDENT "`json:" STRING "`"
    ;

FunctionDecl: "fn" IDENT "(" ParamList ")" "->" IDENT Block
    | "fn" IDENT "." IDENT "(" ParamList ")" "->" IDENT Block
    ;

ParamList: Param ("," Param)* 
    ;

Param: IDENT IDENT
    ;

Block: "{" StatementList "}"
    ;

StatementList: Statement*
    ;

Statement: "var" IDENT IDENT "=" Expr ";"
    | "return" Expr ";"
    ;

Expr: IDENT
    | STRING
    | NUMBER
    | Expr "." IDENT
    | Expr "(" ExprList ")"
    | "match" Expr "{" MatchArmList "}"
    ;

ExprList: Expr ("," Expr)* 
    ;

MatchArmList: MatchArm ("," MatchArm)* 
    ;

MatchArm: Expr "->" Expr
    ;

IDENT: r"[a-zA-Z_][a-zA-Z0-9_]*"
    ;

STRING: r#""([^"\\]|\\.)*""#
    ;

NUMBER: r"[0-9]+"
    ;

