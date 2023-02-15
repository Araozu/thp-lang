# Grammar

## Module

A module is (commonly) a single source file.

- `module = variable_binding*`


### `variable_binding`

A declaration with `var` or `val`.

```ebnf
var = "var"
val = "val"
variable_binding = (var | val), identifier, "=", expression
```


### `expression`

For now just a number, string or boolean

```ebnf
expression = number | string | boolean
```


## Type annotations

```ebnf
variable_binding = Datatype, (var | val), identifier, "=", expression
```

