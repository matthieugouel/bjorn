# Bjørn Grammar

```
program: (NEWLINE | statement)* EOF

statement: simple_statement
         | compound_statement

simple_statement: expression_statement NEWLINE

compound_statement: if_statement
                  | while_statement

if_statement: 'if' logical_or_expr ':' bloc ('else' 'if' logical_or_expr ':' bloc)* ['else' ':' bloc]

while_statement: 'while' logical_or_expr ':' bloc

bloc: NEWLINE INDENT statement+ DEDENT

expression_statement: logical_or_expr ['=' logical_or_expr]

logical_or_expr: logical_and_expr ('or' logical_and_expr)*

logical_and_expr: logical_not_expr ('and' logical_not_expr)*

logical_not_expr: 'not' logical_not_expr
                | comparison

comparison: expr (('==' | '!=' | '<=' | '>=' | '<' | '>') expr)*

expr: term (('+' | '-') term)*

term: atom (('*' | '/') atom)*

atom: INT_NUMBER
    | FLOAT_NUMBER
    | PLUS atom
    | MINUS atom
    | '(' expr ')'
    | TRUE
    | FALSE
    | variable
```
