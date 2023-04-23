use std::str::Chars;

use itertools::{Itertools, MultiPeek};

use crate::token::{Token, TokenType};

static KEYWORDS: phf::Map<&'static str, TokenType> = phf::phf_map! {
    "and" => TokenType::And,
    "class" => TokenType::Class,
    "else" => TokenType::Else,
    "false" => TokenType::False,
    "for" => TokenType::For,
    "fun" => TokenType::Fun,
    "if" => TokenType::If,
    "nil" => TokenType::Nil,
    "or" => TokenType::Or,
    "print" => TokenType::Print,
    "return" => TokenType::Return,
    "super" => TokenType::Super,
    "this" => TokenType::This,
    "true" => TokenType::True,
    "var" => TokenType::Var,
    "while" => TokenType::While,
};

struct Scanner<'a> {
    source: &'a str,
    chars: MultiPeek<Chars<'a>>,
    line: usize,
    start: usize,
    current: usize,
}

impl<'a> Scanner<'a> {
    fn new(source: &'a str) -> Self {
        Self {
            source,
            chars: source.chars().multipeek(),
            line: 1,
            start: 0,
            current: 0,
        }
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.chars.next()?;
        self.current += 1;
        Some(ch)
    }

    fn new_token(&mut self, token_type: TokenType) -> Token {
        let from = self.start;
        let to = self.current;
        self.start = to;
        Token::new(token_type, &self.source[from..to], self.line)
    }

    fn next_matches(&mut self, next: char) -> bool {
        match self.chars.peek() {
            Some(ch) => {
                if ch == &next {
                    self.advance();
                    true
                } else {
                    false
                }
            }
            None => false,
        }
    }

    fn ignore_until_new_line(&mut self) {
        while let Some(ch) = self.chars.peek() {
            self.start = self.current;
            if ch == &'\n' {
                break;
            } else {
                self.advance();
            }
        }
    }

    fn read_string(&mut self) -> Token {
        while let Some(ch) = self.chars.peek() {
            if ch == &'"' {
                self.advance();
                break;
            } else {
                let ch = self.advance().unwrap();
                if ch == '\n' {
                    self.line += 1;
                }
            }
        }

        let lexeme = &self.source[self.start..self.current];
        // trim the surrounding quotes
        let literal = &lexeme[1..lexeme.len() - 1];
        self.new_token(TokenType::String(literal.into()))
    }

    fn read_number(&mut self) -> Token {
        while let Some(ch) = self.chars.peek() {
            if ch.is_ascii_digit() {
                self.advance();
            } else if ch == &'.' {
                if let Some(ch2) = self.chars.peek() {
                    if ch2.is_ascii_digit() {
                        self.advance();
                        self.advance();
                    }
                }
            } else {
                break;
            }
        }
        self.new_token(TokenType::Number(
            self.source[self.start..self.current].parse().unwrap(),
        ))
    }

    fn read_identifier(&mut self) -> Token {
        while let Some(ch) = self.chars.peek() {
            if ch.is_alphanumeric() || ch == &'_' {
                self.advance();
            } else {
                break;
            }
        }

        let identifier = &self.source[self.start..self.current];
        if let Some(keyword) = KEYWORDS.get(&identifier) {
            self.new_token(keyword.to_owned())
        } else {
            self.new_token(TokenType::Identifier(identifier.into()))
        }
    }
}

pub fn scan_tokens(source: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut scanner = Scanner::new(source);

    tokens.extend(std::iter::from_fn(move || loop {
        let ch = scanner.advance()?;

        match ch {
            '(' => return Some(scanner.new_token(TokenType::LeftParen)),
            ')' => return Some(scanner.new_token(TokenType::RightParen)),
            '{' => return Some(scanner.new_token(TokenType::LeftBrace)),
            '}' => return Some(scanner.new_token(TokenType::RightBrace)),
            ',' => return Some(scanner.new_token(TokenType::Comma)),
            '.' => return Some(scanner.new_token(TokenType::Dot)),
            '-' => return Some(scanner.new_token(TokenType::Minus)),
            '+' => return Some(scanner.new_token(TokenType::Plus)),
            ';' => return Some(scanner.new_token(TokenType::Semicolon)),
            '*' => return Some(scanner.new_token(TokenType::Star)),
            '!' => {
                if scanner.next_matches('=') {
                    return Some(scanner.new_token(TokenType::BangEqual));
                } else {
                    return Some(scanner.new_token(TokenType::Bang));
                }
            }
            '=' => {
                if scanner.next_matches('=') {
                    return Some(scanner.new_token(TokenType::EqualEqual));
                } else {
                    return Some(scanner.new_token(TokenType::Equal));
                }
            }
            '<' => {
                if scanner.next_matches('=') {
                    return Some(scanner.new_token(TokenType::LessEqual));
                } else {
                    return Some(scanner.new_token(TokenType::Less));
                }
            }
            '>' => {
                if scanner.next_matches('=') {
                    return Some(scanner.new_token(TokenType::GreaterEqual));
                } else {
                    return Some(scanner.new_token(TokenType::Greater));
                }
            }
            '/' => {
                if scanner.next_matches('/') {
                    // comment, ignore the rest of the line
                    scanner.ignore_until_new_line();
                } else {
                    return Some(scanner.new_token(TokenType::Slash));
                }
            }
            ' ' | '\t' | '\r' => scanner.start += 1,
            '\n' => {
                scanner.line += 1;
                scanner.start += 1;
            }
            // TODO: report error on unterminated string
            // TODO: do not trim when unterminated string
            '"' => return Some(scanner.read_string()),
            // TODO: handle number parsing error
            d if d.is_ascii_digit() => return Some(scanner.read_number()),
            a if a.is_alphabetic() || a == '_' => return Some(scanner.read_identifier()),
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
        let source = "/////  \n\
            /*//*-\n\
            +";
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
        let source = "\"\"\"string\"\"first\n\
            second\"";
        let tokens = scan_tokens(source);
        let expected_tokens = vec![
            Token::new(TokenType::String("".into()), "\"\"", 1),
            Token::new(TokenType::String("string".into()), "\"string\"", 1),
            Token::new(
                TokenType::String("first\nsecond".into()),
                "\"first\nsecond\"",
                2,
            ),
            Token::new(TokenType::Eof, "", 2),
        ];
        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn numbers() {
        let source = "123\n\
            123.456\n\
            .456\n\
            123.";
        let tokens = scan_tokens(source);
        let expected_tokens = vec![
            Token::new(TokenType::Number(123.0), "123", 1),
            Token::new(TokenType::Number(123.456), "123.456", 2),
            Token::new(TokenType::Dot, ".", 3),
            Token::new(TokenType::Number(456.0), "456", 3),
            Token::new(TokenType::Number(123.0), "123", 4),
            Token::new(TokenType::Dot, ".", 4),
            Token::new(TokenType::Eof, "", 4),
        ];
        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn identifiers() {
        let source = "andy formless fo _ _123 _abc ab_123\n\
            abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890_";
        let tokens = scan_tokens(source);
        let expected_tokens = vec![
            Token::new(TokenType::Identifier("andy".into()), "andy", 1),
            Token::new(TokenType::Identifier("formless".into()), "formless", 1),
            Token::new(TokenType::Identifier("fo".into()), "fo", 1),
            Token::new(TokenType::Identifier("_".into()), "_", 1),
            Token::new(TokenType::Identifier("_123".into()), "_123", 1),
            Token::new(TokenType::Identifier("_abc".into()), "_abc", 1),
            Token::new(TokenType::Identifier("ab_123".into()), "ab_123", 1),
            Token::new(
                TokenType::Identifier(
                    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890_".into(),
                ),
                "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890_",
                2,
            ),
            Token::new(TokenType::Eof, "", 2),
        ];
        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn keywords() {
        let source =
            "and class else false for fun if nil or print return super this true var while";
        let tokens = scan_tokens(source);
        let expected_tokens = vec![
            Token::new(TokenType::And, "and", 1),
            Token::new(TokenType::Class, "class", 1),
            Token::new(TokenType::Else, "else", 1),
            Token::new(TokenType::False, "false", 1),
            Token::new(TokenType::For, "for", 1),
            Token::new(TokenType::Fun, "fun", 1),
            Token::new(TokenType::If, "if", 1),
            Token::new(TokenType::Nil, "nil", 1),
            Token::new(TokenType::Or, "or", 1),
            Token::new(TokenType::Print, "print", 1),
            Token::new(TokenType::Return, "return", 1),
            Token::new(TokenType::Super, "super", 1),
            Token::new(TokenType::This, "this", 1),
            Token::new(TokenType::True, "true", 1),
            Token::new(TokenType::Var, "var", 1),
            Token::new(TokenType::While, "while", 1),
            Token::new(TokenType::Eof, "", 1),
        ];
        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn whitespaces() {
        let source = "space    tabs				newlines \n\n cr\r\rend";
        let tokens = scan_tokens(source);
        let expected_tokens = vec![
            Token::new(TokenType::Identifier("space".into()), "space", 1),
            Token::new(TokenType::Identifier("tabs".into()), "tabs", 1),
            Token::new(TokenType::Identifier("newlines".into()), "newlines", 1),
            Token::new(TokenType::Identifier("cr".into()), "cr", 3),
            Token::new(TokenType::Identifier("end".into()), "end", 3),
            Token::new(TokenType::Eof, "", 3),
        ];
        assert_eq!(tokens, expected_tokens);
    }
}
