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
function declaration = "fun", identifier, params list, return type?, block
```

### Params list

```ebnf
params list = "(", ")"
```

### Return type

```ebnf
return type = ;
```


### Block

```ebnf
block = "{", (statement, (new line, statement)*)?,  "}"
```


### Statement

```ebnf
statement = function call
```


## Function call

```ebnf
function call = identifier, arguments list
```


### Arguments list

```ebnf
arguments list = "(", ")"
```


