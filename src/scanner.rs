use std::{iter::Peekable, str::CharIndices};

use crate::token::{Token, TokenType};

pub struct Scanner {
    source: String,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self { source }
    }

    pub fn scan_tokens(&self) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut line = 1;
        // let mut start = 0;
        // let mut current = 0;

        let mut chars = self.source.char_indices().peekable();
        tokens.extend(std::iter::from_fn(move || loop {
            let (idx, c) = chars.next()?;

            match c {
                '(' => {
                    return Some(Token::new(
                        TokenType::LeftParen,
                        self.source[idx..idx + c.len_utf8()].to_string(),
                        line,
                    ))
                }
                ')' => return Some(Token::new(TokenType::RightParen, c.to_string(), line)),
                '{' => return Some(Token::new(TokenType::LeftBrace, c.to_string(), line)),
                '}' => return Some(Token::new(TokenType::RightBrace, c.to_string(), line)),
                ',' => return Some(Token::new(TokenType::Comma, c.to_string(), line)),
                '.' => return Some(Token::new(TokenType::Dot, c.to_string(), line)),
                '-' => return Some(Token::new(TokenType::Minus, c.to_string(), line)),
                '+' => return Some(Token::new(TokenType::Plus, c.to_string(), line)),
                ';' => return Some(Token::new(TokenType::Semicolon, c.to_string(), line)),
                '*' => return Some(Token::new(TokenType::Star, c.to_string(), line)),
                // '!' => match chars.peek() {
                //     Some((_, next)) => {
                //         if next == &'=' {
                //             chars.next().unwrap();
                //             return Some(Token::new(TokenType::BangEqual, c.to_string(), line));
                //         }
                //     }
                //     None => return Some(Token::new(TokenType::Bang, c.to_string(), line)),
                // },
                '!' => {
                    if Self::next_matches(&mut chars, '=') {
                        return Some(Token::new(TokenType::BangEqual, c.to_string(), line));
                    } else {
                        return Some(Token::new(TokenType::Bang, c.to_string(), line));
                    }
                }
                ' ' | '\t' => (),
                '\n' => line += 1,
                _ => {
                    // report error
                    ()
                }
            }
            continue;
        }));

        tokens.push(Token::new(TokenType::Eof, "".into(), line));
        tokens
    }

    fn next_matches(chars: &mut Peekable<CharIndices>, next: char) -> bool {
        match chars.peek() {
            Some((_, value)) => {
                if value == &next {
                    chars.next().unwrap();
                    true
                } else {
                    false
                }
            }
            None => false,
        }
    }
}
