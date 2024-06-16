use crate::lexic::token::Token;
use crate::lexic::token::TokenType::{NewLine, INDENT};

/// Attempts to parse a binary operator and handles indentation
///
/// Binary operators may be in a new line as long as they are indented.
/// The new line may be before or after the operator.
///
/// Once an operator is indented, all following operators completely disregard newline/indentation
/// until a matching dedent is found.
pub fn try_binary_op<'a>(
    tokens: &'a Vec<Token>,
    pos: usize,
    operators: Vec<&str>,
    indentation_level: u32,
) -> Option<(&'a Token, usize, u32)> {
    let mut indent_count = 0;

    // handle possible opening indentation
    let pos = match (tokens.get(pos), tokens.get(pos + 1)) {
        // New indentation level
        (Some(t1), Some(t2)) if t1.token_type == NewLine && t2.token_type == INDENT => {
            indent_count += 1;
            pos + 2
        }
        // when indented, ignore newlines
        (Some(t), _) if t.token_type == NewLine && indentation_level > 0 => pos + 1,
        // let other handlers handle this
        _ => pos,
    };

    // try to parse binary operator
    let (matched_token, pos) = match tokens.get(pos) {
        Some(token) if operators.contains(&token.value.as_str()) => (token, pos + 1),
        _ => return None,
    };

    // handle possible closing indentation
    let pos = match (tokens.get(pos), tokens.get(pos + 1)) {
        // New indentation level
        (Some(t1), Some(t2)) if t1.token_type == NewLine && t2.token_type == INDENT => {
            indent_count += 1;
            pos + 2
        }
        // when indented, ignore newlines
        (Some(t), _) if t.token_type == NewLine && indentation_level > 0 => pos + 1,
        // let other handlers handle this
        _ => pos,
    };

    // return the matched token, next position and new indentation level
    Some((matched_token, pos, indent_count))
}

/// macro for handling indentation in expressions
#[macro_export]
macro_rules! handle_indentation {
    ($tokens: ident, $next_pos: ident, $indent_count: ident, $indentation_level: ident) => {
        match ($tokens.get($next_pos), $tokens.get($next_pos + 1)) {
            // New indentation level
            (Some(t1), Some(t2))
                if t1.token_type == TokenType::NewLine && t2.token_type == TokenType::INDENT =>
            {
                // set indentation
                $next_pos += 2;
                $indent_count += 1;
            }
            // we are indented, ignore newlines
            (Some(t), _) if t.token_type == TokenType::NewLine && $indentation_level > 0 => {
                $next_pos += 1;
            }
            // let other handlers handle this
            _ => {}
        };
    };
}

/// macro for handling dedentation in expressions
#[macro_export]
macro_rules! handle_dedentation {
    ($tokens: ident, $next_pos: ident, $indent_count: ident) => {
        for _ in 0..$indent_count {
            // Expect a DEDENT for each indentation matched
            match $tokens.get($next_pos) {
                // continue
                Some(t) if t.token_type == TokenType::DEDENT => {}
                // This should be unreachable, as the lexer always emits a DEDENT for each INDENT
                _ => unreachable!(
                    "Illegal parser state: Expected DEDENT (count: {})",
                    $indent_count
                ),
            };

            $next_pos += 1;
        }
    };
}
