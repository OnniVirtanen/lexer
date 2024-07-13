pub fn is_whitespace(character: char) -> bool {
    match character {
        ' ' | '\t' | '\n' | '\r' => return true,
        _ => return false,
    }
}

pub fn is_separator(character: char) -> bool {
    match character {
        ';' | ',' | '(' | ')' | '[' | ']' | '{' | '}' | '.' | ':' => return true,
        _ => return false,
    }
}

pub fn is_operator(character: char) -> bool {
    match character {
        '+' | '-' | '*' | '/' | '%' | '&' | '|' | '!' | '=' | '<' | '>' => return true,
        _ => return false,
    }
}

pub fn is_beginning_of_char(character: char) -> bool {
    return character == '\'';
}

pub fn is_beginning_of_string(character: char) -> bool {
    return character == '"';
}

pub fn is_number(character: char) -> bool {
    match character {
        '0'..='9' => return true,
        _ => return false,
    }
}

pub fn is_letter(character: char) -> bool {
    match character {
        _ if character.is_alphabetic() => return true,
        _ => return false,
    }
}

pub fn is_keyword(string: String) -> bool {
    match string.as_str() {
        "if" | "else" | "while" | "int" | "float" | "bool" | "char" | "string" | "fun" | "return" | "void" => return true,
        _ => return false,
    }
}