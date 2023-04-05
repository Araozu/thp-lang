# Changelog

## TODO

- [ ] Automatic semicolon insertion
- [ ] Improve error messages
- [ ] Parse other language constructions
- [ ] Type checking
- [ ] Check for conflicting identifiers
- [ ] Namespace identifiers in the symbol table
- [ ] Stdlib
- [ ] Document code

## v0.0.5

- Scan single line comments
- Refactor String token to include double quotes (") in its content

## v0.0.4

- Explicit datatype of variables
- Improve error messages when a syntax error is found (show offending line and offending token)
- Show different error messages for val/var binding

## v0.0.3

- Get datatype of an identifier from the symbol table
- Improve documentation of the code
- Simple ASI: insert semicolon after a single or series of new lines
- The token stream now always ends with a Semicolon and EOF token, regardless of input

## v0.0.2

- Compilation of `val` and `var` bindings with a number, string or boolean as value.
- Register symbols and datatypes in the Symbol table.
- Add better error messages for lexical errors. Show:
    - Offending line
    - Pointer to offending character
    - Error message


## v0.0.1

- Compilation of a `val` binding with a number.
- Scan all tokens except new lines, indentation.
