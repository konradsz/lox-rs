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
        let (idx, ch) = chars.next()?;
        state.current = idx;

        match ch {
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
            '"' => {
                // TODO: report error on unterminated string
                let literal = read_string(&mut chars, &mut state);
                // TODO: do not trim when unterminated string
                let mut token = new_token(TokenType::String(literal), source, &mut state);
                token.lexeme = token.lexeme[1..token.lexeme.len() - 1].to_string();

                return Some(token);
            }
            d if d.is_digit(10) => {
                let mut number = String::from(d);
                number += &read_number(&mut chars, &mut state);
                let number = number.parse().unwrap(); // TODO: handle parsing error
                return Some(new_token(TokenType::Number(number), source, &mut state));
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
        Some((_, ch)) => {
            if ch == &next {
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
    while let Some((_, ch)) = chars.peek() {
        state.current += 1;
        state.start = state.current;
        if ch == &'\n' {
            break;
        } else {
            chars.next();
        }
    }
}

fn read_string(chars: &mut Peekable<CharIndices>, state: &mut State) -> String {
    let mut literal = String::new();
    while let Some((_, ch)) = chars.peek() {
        state.current += 1;
        if ch == &'"' {
            chars.next();
            break;
        } else {
            let (_, ch) = chars.next().unwrap();
            literal.push(ch);
            if ch == '\n' {
                state.line += 1;
            }
        }
    }
    literal
}

fn read_number(chars: &mut Peekable<CharIndices>, state: &mut State) -> String {
    let mut literal = String::new();
    while let Some((_, ch)) = chars.peek() {
        if ch.is_digit(10) || ch == &'.' {
            state.current += 1;
            let (_, ch) = chars.next().unwrap();
            literal.push(ch);
        } else {
            break;
        }
    }
    literal
}

#[cfg(test)]
mod tests {
    use crate::token::{Token, TokenType};

    use super::scan_tokens;

    #[test]
    fn punctuators() {
        let source = "( ){};,+-*!===<=>=!=<>/.";
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

    #[test]
    fn string_literals() {
        let source = "\"\"\"string\"\"first\nsecond\"";
        let tokens = scan_tokens(source);
        let expected_tokens = vec![
            Token::new(TokenType::String("".into()), "", 1),
            Token::new(TokenType::String("string".into()), "string", 1),
            Token::new(
                TokenType::String("first\nsecond".into()),
                "first\nsecond",
                2,
            ),
            Token::new(TokenType::Eof, "", 2),
        ];
        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn numbers() {
        let source = "123\n123.456\n.456\n123.";
        let tokens = scan_tokens(source);
        let expected_tokens = vec![
            Token::new(TokenType::Number(123.0), "123", 1),
            Token::new(TokenType::Number(123.456), "123.456", 2),
            Token::new(TokenType::Dot, ".", 3),
            Token::new(TokenType::Number(456.0), "456", 3),
            Token::new(TokenType::Number(123.0), "123.", 4),
            Token::new(TokenType::Eof, "", 4),
        ];
        assert_eq!(tokens, expected_tokens);
    }
}
