use crate::{
    expr::{Expr, LiteralType},
    token::Token,
    visitor::{self, Visitor},
};

pub struct AstPrinter;

impl AstPrinter {
    fn parenthesize(&mut self, name: &str, expressions: &[&Expr]) -> String {
        let mut output = String::new();
        output.push('(');
        output.push_str(name);

        for expr in expressions {
            output.push(' ');
            output.push_str(&visitor::walk_expr(self, expr));
        }

        output.push(')');
        output
    }
}

impl Visitor for AstPrinter {
    type Output = String;

    fn visit_binary_expr(&mut self, left: &Expr, operator: &Token, right: &Expr) -> String {
        self.parenthesize(&operator.lexeme, &[left, right])
    }

    fn visit_grouping_expr(&mut self, expression: &Expr) -> String {
        self.parenthesize("group", &[expression])
    }

    fn visit_literal_expr(&mut self, value: &LiteralType) -> String {
        match value {
            LiteralType::String(s) => s.to_string(),
            LiteralType::Number(n) => n.to_string(),
            LiteralType::Boolean(b) => b.to_string(),
            LiteralType::Null => String::new(),
        }
    }

    fn visit_unary_expr(&mut self, operator: &Token, right: &Expr) -> String {
        self.parenthesize(&operator.lexeme, &[right])
    }
}
