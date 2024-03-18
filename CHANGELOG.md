# Changelog

## TODO

- Implement AST transformation before codegen:
    Create a new AST to represent PHP source code
    and a THP ast -> PHP ast process, so that the
    codegen section can focus only in codegen, not in
    translation of thp->php.
- Parse __more__ binary operators
- Parse more complex bindings
- Watch mode
- Improve error messages
- Parse other language constructions
- Type checking
- Check for conflicting identifiers
- Namespace identifiers in the symbol table
- Stdlib
- Document code
- Watch mode
- Formatter
- Simple language server
- Decide how to handle comments in the syntax (?)(should comments mean something like in rust?)

## v0.0.12

- [ ] Infer datatype of a function call expression
- [ ] Infer datatype of binary operators
- [ ] Execute semantic analysis on the function's block
- [ ] Write tests


## v0.0.11

- [ ] Parse binding of form `val Type variable = value`
- [ ] Parse binding of form `Type variable = value`
- [ ] Infer datatype of `value` in the above for a simple expression
- [ ] Ensure that the anotated datatype matches the datatype of `value` in the above
- [ ] Infer datatype of a `val variable = value` in the AST: Use the infered datatype
- [ ] Formally define the top level constructs
- [ ] Parse bindings and function declarations as top level constructs
- [ ] Parse function declaration arguments (`Type id`)
- [ ] Parse function return datatype (`fun f() -> Type`)
- [x] Return parsing to variables to var/val
- [ ] Write tests


## v0.0.10

- [x] Parse function call parameters
- [x] Codegen function call parameters
- [x] Begin work on semantic analysis
- [x] Minimal symbol table
- [x] Check duplicate function declarations
- [x] Improve REPL/File compilation code
- [x] Check binding duplication in it's scope
- [x] Check function duplication in it's scope
- [x] Transform simple THP expression into PHP statements

## v0.0.9

- [x] Hand-make CLI, remove clap
- [x] Compile a single file
- [x] Display error messages during compilation instead of panicking
- [x] Improve error messages
- [x] Implement code generation for ast nodes implemented as of now



## v0.0.8

- Parse block of code
- Parse multiple statements inside a block
- Parse unary operator (`!` & `-`)
- Parse binary operators


## v0.0.7

- Parse minimal function declarations following a grammar
- Parse function call, binding as statement
- Parse a statement as body of a function declaration


## v0.0.6

- Parse function declarations
- Parse multiple function declarations
- Parse multiple bindings
- Compile from file
- Emit INDENT & DEDENT tokens


## v0.0.5

- Scan single line comments
- Refactor String token to include double quotes (") in its content
- Refactor datachecking of semantic analysis

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
