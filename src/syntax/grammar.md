# Grammar


## Source file

```ebnf
source file = top level statement*
```


## Top level statement

Current focus: Have a mvp compiler (w lexical/syntactic/semantic analysis + codegen) for
simple function calls, and then implement other features top down

```ebnf
top level statement = expression
                    | function declaration
```


## Function declaration

```ebnf
function declaration = "fun", identifier, params list, return type?, block;

params list = "(", ")";

return type = ;
```


### Block

```ebnf
block = "{", (statement, (new line, statement)*)?, "}"
```


### Statement

```ebnf
statement = binding
          | function call
```


## Function call

```ebnf
function call = identifier, arguments list;

arguments list = "(", ")"
```


## Binding

```ebnf
binding = ("val" | "var"), identifier, "=", expression
```


## Operator precedence

From highest to lowest:

- `== !=`, left associative
- `> >= < <=`, left associative
- `- +`, left associative
- `/ *`, left associative
- `! -`, left associative

## Expression

```ebnf
expression = equality;

equality = comparison, (("==" | "!="), comparison )*;

comparison = term, ((">" | ">=" | "<" | "<="), term)*;

term = factor, (("-" | "+"), factor)*;

factor = unary, (("/" | "*"), unary)*;

unary = ("!" | "-"), expression
      | primary;

function call expr = primary, (arguments list)?
                   | primary;

primary = number | string | boolean | identifier | ("(", expression, ")");
```

```thp
primary()
```


