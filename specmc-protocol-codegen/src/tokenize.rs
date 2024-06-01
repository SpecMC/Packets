const SPECIAL_CHARS: &[&str] = &[
    " ", "\t", "\n", "\r", // whitespace will not be included in tokens
    "==", "!=", "||", "&&", "**", "(", ")", "{", "}", "[", "]", ",", "=",
    ";",
    // "-", removed to not mess with negative numbers
    // removed because useless
    // ".",
    // ":",
    // "+",
    // "*",
    // "/",
    // "%",
    // "!",
    // "&",
    // "|",
    // "^",
    // "~",
];

pub fn tokenize(input: &str) -> Vec<String> {
    let mut tokens: Vec<String> = vec![];
    let mut current_token: String = "".to_string();

    let mut i: usize = 0;
    while i < input.len() {
        let ch: char = input.as_bytes()[i] as char;

        let mut found_special_char: bool = false;
        for special_char in SPECIAL_CHARS {
            if input[i..].starts_with(special_char) {
                found_special_char = true;
                if !current_token.is_empty() {
                    tokens.push(current_token);
                    current_token = "".to_string();
                }
                if !special_char.trim().is_empty() {
                    tokens.push(special_char.to_string());
                }
                i += special_char.len() - 1;
                break;
            }
        }

        if !found_special_char {
            current_token.push(ch);
        }

        i += 1;
    }

    tokens
}
