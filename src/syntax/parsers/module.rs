use crate::{
    error_handling::SyntaxError,
    lexic::token::Token,
    syntax::{
        ast::{Expression, ModuleAST, ModuleMembers, Statement},
        parseable::{Parseable, ParsingError, ParsingResult},
    },
};

impl<'a> Parseable<'a> for ModuleAST<'a> {
    type Item = ModuleAST<'a>;

    /// Parses a THP module/source file
    ///
    /// As this function parses the whole file, it ignores `current_pos` and
    /// always starts from token 0.
    ///
    /// Its grammar is defined it the spec, at the webpage
    fn try_parse(tokens: &'a Vec<Token>, current_pos: usize) -> ParsingResult<'a, Self::Item> {
        let mut productions = Vec::<ModuleMembers>::new();
        let tokens_len = tokens.len();
        let mut current_pos = 0;

        // Minus one because last token is EOF
        // TODO: Does that EOF do anything?
        while current_pos < tokens_len - 1 {
            // Attempt to parse an statement
            match Statement::try_parse(tokens, current_pos) {
                Ok((prod, next_pos)) => {
                    productions.push(ModuleMembers::Stmt(prod));
                    current_pos = next_pos;
                    continue;
                }
                Err(ParsingError::Err(error)) => {
                    // TODO: Better error handling, write a better error message
                    return Err(ParsingError::Err(error));
                }
                _ => {}
            };

            // Attempt to parse an expression
            match Expression::try_parse(tokens, current_pos) {
                Ok((prod, next_pos)) => {
                    productions.push(ModuleMembers::Expr(prod));
                    current_pos = next_pos;
                }
                Err(ParsingError::Err(error)) => {
                    // TODO: Better error handling, write a better error message
                    return Err(ParsingError::Err(error));
                }
                _ => {}
            }

            // If we reached this point we didn't match any productions and fail
            let t = &tokens[current_pos];

            return Err(ParsingError::Err(SyntaxError {
                error_start: t.position,
                error_end: t.get_end_position(),
                reason: "Expected an statement or an expresion at the top level.".into(),
            }));
        }

        Ok((ModuleAST { productions }, current_pos))
    }
}
