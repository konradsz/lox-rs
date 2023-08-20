use rlox::{
    expr::{Expr, LiteralType},
    parser::Parser,
    scanner,
    token::{Token, TokenType},
};

#[test]
fn parse_expression() {
    let expr_string = "(5 - (3 - 1)) + -1";
    let tokens = scanner::scan_tokens(expr_string);
    let mut parser = Parser::new(&tokens);
    let expr = Parser::parse(&mut parser);

    assert_eq!(
        expr,
        Expr::Binary {
            left: Box::new(Expr::Grouping {
                expression: Box::new(Expr::Binary {
                    left: Box::new(Expr::Literal {
                        value: LiteralType::Number(5.0)
                    }),
                    operator: Token::new(TokenType::Minus, "-", 1),
                    right: Box::new(Expr::Grouping {
                        expression: Box::new(Expr::Binary {
                            left: Box::new(Expr::Literal {
                                value: LiteralType::Number(3.0)
                            }),
                            operator: Token::new(TokenType::Minus, "-", 1),
                            right: Box::new(Expr::Literal {
                                value: LiteralType::Number(1.0)
                            })
                        })
                    })
                })
            }),
            operator: Token::new(TokenType::Plus, "+", 1),
            right: Box::new(Expr::Unary {
                operator: Token::new(TokenType::Minus, "-", 1),
                right: Box::new(Expr::Literal {
                    value: LiteralType::Number(1.0)
                })
            })
        }
    )
}
