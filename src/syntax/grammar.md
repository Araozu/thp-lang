# Grammar


## Module

A module is (commonly) a single source file.

```ebnf
module = top level declaration*
```


## Top level declaration

```ebnf
top level declaration = function declaration
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

function call = primary, (arguments list)?;

primary = number | string | boolean | identifier | ("(", expression, ")");
```




