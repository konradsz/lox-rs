use std::{iter::Peekable, str::CharIndices};

use crate::token::{Token, TokenType};

struct State {
    line: usize,
    start: usize,
    current: usize,
}

pub fn scan_tokens(source: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut state = State {
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
            '<' => {
                if next_matches(&mut chars, '=', &mut state) {
                    return Some(new_token(TokenType::LessEqual, source, &mut state));
                } else {
                    return Some(new_token(TokenType::Less, source, &mut state));
                }
            }
            '>' => {
                if next_matches(&mut chars, '=', &mut state) {
                    return Some(new_token(TokenType::GreaterEqual, source, &mut state));
                } else {
                    return Some(new_token(TokenType::Greater, source, &mut state));
                }
            }
            '/' => {
                if next_matches(&mut chars, '/', &mut state) {
                    // comment, ignore the rest of the line
                    ignore_until_new_line(&mut chars, &mut state);
                } else {
                    return Some(new_token(TokenType::Slash, source, &mut state));
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

    tokens.push(Token::new(TokenType::Eof, "", source.lines().count()));
    tokens
}

fn new_token(token_type: TokenType, source: &str, state: &mut State) -> Token {
    let from = state.start;
    let to = state.current;
    state.start = to + 1; // move start position to the next character right after the token
    Token::new(token_type, &source[from..=to], state.line)
}

fn next_matches(chars: &mut Peekable<CharIndices>, next: char, state: &mut State) -> bool {
    match chars.peek() {
        Some((_, c)) => {
            if c == &next {
                chars.next();
                state.current += 1;
                true
            } else {
                false
            }
        }
        None => false,
    }
}

fn ignore_until_new_line(chars: &mut Peekable<CharIndices>, state: &mut State) {
    while let Some((_, c)) = chars.peek() {
        state.current += 1;
        state.start = state.current;
        if c == &'\n' {
            break;
        } else {
            chars.next();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::token::{Token, TokenType};

    use super::scan_tokens;

    #[test]
    fn punctuators() {
        let source = "(){};,+-*!===<=>=!=<>/.";
        let tokens = scan_tokens(source);
        let expected_tokens = vec![
            Token::new(TokenType::LeftParen, "(", 1),
            Token::new(TokenType::RightParen, ")", 1),
            Token::new(TokenType::LeftBrace, "{", 1),
            Token::new(TokenType::RightBrace, "}", 1),
            Token::new(TokenType::Semicolon, ";", 1),
            Token::new(TokenType::Comma, ",", 1),
            Token::new(TokenType::Plus, "+", 1),
            Token::new(TokenType::Minus, "-", 1),
            Token::new(TokenType::Star, "*", 1),
            Token::new(TokenType::BangEqual, "!=", 1),
            Token::new(TokenType::EqualEqual, "==", 1),
            Token::new(TokenType::LessEqual, "<=", 1),
            Token::new(TokenType::GreaterEqual, ">=", 1),
            Token::new(TokenType::BangEqual, "!=", 1),
            Token::new(TokenType::Less, "<", 1),
            Token::new(TokenType::Greater, ">", 1),
            Token::new(TokenType::Slash, "/", 1),
            Token::new(TokenType::Dot, ".", 1),
            Token::new(TokenType::Eof, "", 1),
        ];
        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn comments() {
        let source = "/////  \n/*//*-\n+";
        let tokens = scan_tokens(source);
        let expected_tokens = vec![
            Token::new(TokenType::Slash, "/", 2),
            Token::new(TokenType::Star, "*", 2),
            Token::new(TokenType::Plus, "+", 3),
            Token::new(TokenType::Eof, "", 3),
        ];
        assert_eq!(tokens, expected_tokens);
    }
}
