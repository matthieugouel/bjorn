# Bjørn Grammar

```
program: NEWLINE
       | expression_statement

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
