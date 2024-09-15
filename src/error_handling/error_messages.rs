//! Contains constants that point to error messages

pub const LEX_INCOMPLETE_STRING: u32 = 0;
pub const LEX_INVALID_HEX_NUMBER: u32 = 1;
pub const LEX_INVALID_OCTAL_NUMBER: u32 = 2;
pub const LEX_INVALID_BINARY_NUMBER: u32 = 3;
pub const LEX_INVALID_FLOATING_NUMBER: u32 = 4;
pub const LEX_INVALID_SCIENTIFIC_NUMBER: u32 = 5;
pub const LEX_INCOMPLETE_MULTILINE_COMMENT: u32 = 6;
pub const SYNTAX_INCOMPLETE_STATEMENT: u32 = 7;
pub const SYNTAX_UNEXPECTED_TOKENS: u32 = 8;
pub const SYNTAX_INCOMPLETE_ARGUMENT_LIST: u32 = 9;
pub const SYNTAX_INVALID_VARIABLE_DECLARATION: u32 = 10;
pub const SYNTAX_INCOMPLETE_PARAMETER_LIST: u32 = 11;
pub const SYNTAX_INVALID_PARAMETER_DECLARATION: u32 = 12;
pub const SYNTAX_INVALID_WHILE_LOOP: u32 = 13;
pub const SYNTAX_INVALID_FUNCTION_DECLARATION: u32 = 14;
pub const SYNTAX_INVALID_FOR_LOOP: u32 = 15;
pub const SYNTAX_INVALID_IF_CONDITION: u32 = 16;
pub const SYNTAX_INCOMPLETE_BLOCK: u32 = 17;

/// Reads the error codes from the error code list
pub fn error_code_to_string() -> String {
    todo!()
}
