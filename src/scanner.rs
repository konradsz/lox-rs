use std::{iter::Peekable, str::CharIndices};

use crate::token::{Token, TokenType};

struct ScannerState {
    line: usize,
    start: usize,
    current: usize,
}

pub fn scan_tokens(source: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut state = ScannerState {
        line: 1,
        start: 0,
        current: 0,
    };

    let mut chars = source.char_indices().peekable();
    tokens.extend(std::iter::from_fn(move || loop {
        let (idx, c) = chars.next()?;
        state.current = idx;

        match c {
            '(' => return Some(new_token(TokenType::LeftParen, source, &mut state)),
            ')' => return Some(new_token(TokenType::RightParen, source, &mut state)),
            '{' => return Some(new_token(TokenType::LeftBrace, source, &mut state)),
            '}' => return Some(new_token(TokenType::RightBrace, source, &mut state)),
            ',' => return Some(new_token(TokenType::Comma, source, &mut state)),
            '.' => return Some(new_token(TokenType::Dot, source, &mut state)),
            '-' => return Some(new_token(TokenType::Minus, source, &mut state)),
            '+' => return Some(new_token(TokenType::Plus, source, &mut state)),
            ';' => return Some(new_token(TokenType::Semicolon, source, &mut state)),
            '*' => return Some(new_token(TokenType::Star, source, &mut state)),
            '!' => {
                if next_matches(&mut chars, '=', &mut state) {
                    return Some(new_token(TokenType::BangEqual, source, &mut state));
                } else {
                    return Some(new_token(TokenType::Bang, source, &mut state));
                }
            }
            '=' => {
                if next_matches(&mut chars, '=', &mut state) {
                    return Some(new_token(TokenType::EqualEqual, source, &mut state));
                } else {
                    return Some(new_token(TokenType::Equal, source, &mut state));
                }
            }
            ' ' | '\t' => state.start += 1,
            '\n' => {
                state.line += 1;
                state.start += 1;
            }
            _ => {
                // report error
                ()
            }
        }
        continue;
    }));

    tokens.push(Token::new(
        TokenType::Eof,
        "".into(),
        source.lines().count(),
    ));
    tokens
}

fn new_token(token_type: TokenType, source: &str, state: &mut ScannerState) -> Token {
    let from = state.start;
    let to = state.current;
    state.start = state.current + 1;
    Token::new(token_type, source[from..=to].to_string(), state.line)
}

fn next_matches(
    chars: &mut Peekable<CharIndices>,
    next: char,
    scanner_state: &mut ScannerState,
) -> bool {
    match chars.peek() {
        Some((_, value)) => {
            if value == &next {
                chars.next().unwrap();
                scanner_state.current += 1;
                true
            } else {
                false
            }
        }
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use crate::token::{Token, TokenType};

    use super::scan_tokens;

    #[test]
    fn punctuators() {
        // let source = "(){};,+-*!===<=>=!=<>/.";
        let source = "()";
        let tokens = scan_tokens(source);
        let expected_tokens = vec![
            Token::new(TokenType::LeftParen, "(".into(), 1),
            Token::new(TokenType::RightParen, ")".into(), 1),
            Token::new(TokenType::Eof, "".into(), 1),
        ];
        assert_eq!(tokens, expected_tokens);
    }
}
