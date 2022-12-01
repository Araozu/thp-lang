
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

pub fn is_lowercase(c: char) -> bool {
    c >= 'a' && c <= 'z'
}

pub fn is_uppercase(c: char) -> bool {
    c >= 'A' && c <= 'Z'
}

pub fn is_identifier_char(c: char) -> bool {
    is_lowercase(c) || is_uppercase(c) || c == '_' || is_digit(c)
}
