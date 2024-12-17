
const RESERVED_LITERALS: &'static [&str] = &[
    "bool",
    "break",
    "const",
    "import",
    "continue",
    "else",
    "false",
    "for",
    "while",
    "if",
    "int",
    "return",
    "len",
    "true",
    "void"
];

pub fn is_whitespace(c: char) -> bool {
    let whitespace = "\n\r\t ";
    return whitespace.contains(c);
}

pub fn is_alphanumeric(c: char) -> bool {
    let numbers = "0123456789";
    let letters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    return numbers.contains(c) || letters.contains(c);
}

pub fn is_reserved_literal(c: &str) -> bool {
    return RESERVED_LITERALS.contains(&c);
}

