
pub fn is_digit(c: char) -> bool {
    '0' <= c && c <= '9'
}

pub fn is_hex_digit(c: char) -> bool {
    is_digit(c) || 'a' <= c && c <= 'f' || 'A' <= c && c <= 'F'
}

pub fn str_append(current: String, c: char) -> String {
    format!("{}{}", current, c)
}

pub fn is_operator(c: char) -> bool {
    c == '+' || c == '-' || c == '=' || c == '*' || c == '!'
             || c == '\\' || c == '/' || c == '|' || c == '@'
             || c == '#' || c == '$' || c == '~' || c == '%' 
             || c == '&' || c == '?' || c == '<' || c == '>' 
             || c == '^' || c == '.' || c == ':'
}

pub fn is_grouping_sign(c: char) -> bool {
    c == '(' || c == ')' || c == '{' || c == '}' || c == '[' || c == ']'
}
