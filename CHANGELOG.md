# Changelog

## TODO

- Test correct operator precedence
- Implement functions as first class citizens
- Parse __more__ binary operators
- Store tokens for the semantic analysis phase, to have actual error reporting
- Parse more complex bindings
- Rework error messages
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
- Not ignore comments & whitespace, for code formatting
- Abstract the parsing of datatypes, such that in the future generics can be implemented in a single place
- Include the original tokens in the AST


## v0.0.15

- [ ] Include comments in the AST
- [ ] Replace all panics with actual errors
- [ ] Remove all old codegen
- [ ] Test codegen
- [ ] Begin work on the code formatter


## v0.0.14

- [x] Define a minimal PHP AST
- [x] Transform THP AST into PHP AST
- [x] Implement minimal codegen for the PHP AST
- [x] Finish the workflow for a hello world


## v0.0.13

- [x] Begin work on a formal grammar
- [x] Simplify/rewrite AST
- [x] Properly parse expression indentation/dedentation
- [x] Define the top level constructs
- [x] Emit INDENT/DEDENT alone instead of NewLine+INDENT/DEDENT
- [x] Refactor code
- [x] Remove `PARSER couldn't parse any construction` error & replace with an actual error message


## v0.0.12

- [x] Infer datatype of an identifier
- [x] Infer datatype of a binary operatior
- [x] Infer datatype of unary operator
- [x] Infer datatype of binary operators
- [x] Infer Int & Float as different types
- [x] Execute semantic analysis on the function's block
- [x] Write tests


## v0.0.11

- [x] Parse binding of form `val Type variable = value`
- [x] Parse binding of form `Type variable = value`
- [x] Infer datatype of `value` in the above for a simple expression (minimal)
- [x] Ensure that the anotated datatype matches the datatype of `value` in the above (binding)
- [x] Infer datatype of a `val variable = value` in the AST: Use the infered datatype (binding)
- [x] Formally define the top level constructs
- [x] Parse bindings and function declarations as top level constructs
- [x] Parse function declaration arguments (`Type id`)
- [x] Parse function return datatype (`fun f() -> Type`)
- [x] Return parsing to variables to var/val
- [x] Write tests


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
