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
