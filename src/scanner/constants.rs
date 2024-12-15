
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
    let whitespace = "\r\t ";
    return whitespace.contains(c);
}

pub fn is_reserved_literal(c: &str) -> bool {
    return RESERVED_LITERALS.contains(&c);
}

