pub fn isWhitespace(character: char) -> bool {
    match character {
        ' ' | '\t' | '\n' | '\r' => return true,
        _ => return false,
    }
}

pub fn isSeparator(character: char) -> bool {
    match character {
        ';' | ',' | '(' | ')' | '[' | ']' | '{' | '}' | '.' | ':' => return true,
        _ => return false,
    }
}

pub fn isOperator(character: char) -> bool {
    match character {
        '+' | '-' | '*' | '/' | '%' | '&' | '|' | '!' | '=' | '<' | '>' => return true,
        _ => return false,
    }
}

pub fn isBeginningOfChar(character: char) -> bool {
    return character == '\'';
}

pub fn isBeginningOfString(character: char) -> bool {
    return character == '"';
}