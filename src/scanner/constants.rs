
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
    let whitespace = "\t ";
    return whitespace.contains(c);
}

pub fn is_reserved_literal(c: &str) -> bool {
    return RESERVED_LITERALS.contains(&c);
}

pub struct ScannerState {
    pub in_multi_line_comment: bool,
    pub in_string: bool,
    pub in_char: bool,
}

