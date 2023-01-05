# Grammar

## Module

A module is (commonly) a single source file.

- `module = variable_binding*`


### `variable_binding`

A declaration with `var` or `val`.

- `var = "var"`
- `val = "val"`
- `variable_binding = (var | val), identifier, "=", expression`

### `expression`

For now just a number

- `expression = number`

