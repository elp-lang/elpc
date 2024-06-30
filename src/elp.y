%start Program
%%

Program: ImportStmts DeclStmts ;

ImportStmts: ImportStmt* ;

ImportStmt: "import" "{" ImportsList "}" "from" STRING ;

ImportsList: Import ("," Import)* ;

Import: IDENT ( "as" IDENT )? ;

DeclStmts: DeclStmt* ;

DeclStmt: EnumDecl
        | ObjectDecl
        | FnDecl
        ;

EnumDecl: "@" IDENT "export" "enum" IDENT "{" EnumVariants "}" ;

EnumVariants: EnumVariant ( "." EnumVariant )* ;

EnumVariant: IDENT "(" "Optional" "<" TYPE ">" ")" | IDENT ;

ObjectDecl: "@" IDENT "export" "object" IDENT ImplementsClause? "{" ObjectBody "}" ;

ImplementsClause: "implements" IDENT ( "," IDENT )* ;

ObjectBody: ObjectField ( ObjectField )* ;

ObjectField: "." IDENT "=" Expr "`json:" STRING "`"
           | "." IDENT TYPE "`json:" STRING "`"
           | "." IDENT "=" Expr
           ;

FnDecl: "fn" IDENT FnSignature Block ;

FnSignature: "(" FnParams ")" "->" TYPE ;

FnParams: FnParam ( "," FnParam )* ;

FnParam: IDENT TYPE ;

Block: "{" Stmt* "}" ;

Stmt: VarDecl
     | ReturnStmt
     | ExprStmt
     ;

VarDecl: "var" IDENT IDENT
       | "const" IDENT IDENT
       ;

ReturnStmt: "return" Expr ;

ExprStmt: Expr ;

Expr: MatchExpr
    | CallExpr
    | IdentExpr
    | Literal
    | SpreadExpr
    ;

MatchExpr: "match" Expr "{" MatchArm ( "," MatchArm )* "}" ;

MatchArm: Range "->" Expr ;

Range: NUMBER ".." NUMBER
     | ".."
     ;

CallExpr: IDENT "." IDENT "(" ExprList? ")" ;

ExprList: Expr ( "," Expr )* ;

IdentExpr: IDENT ;

SpreadExpr: "..." IDENT ;

Literal: STRING
       | NUMBER
       ;

IDENT: /[a-zA-Z_][a-zA-Z0-9_]*/ ;

STRING: /"(?:\\.|[^\\"])*"/ ;

NUMBER: /[0-9]+(\.[0-9]+)?/ ;

TYPE: /[A-Z][a-zA-Z0-9_]*/ ;

