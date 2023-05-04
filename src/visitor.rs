use crate::{
    expr::{Expr, LiteralType},
    token::Token,
};

pub trait Visitor {
    type Output;

    fn visit_binary_expr(&mut self, left: &Expr, operator: &Token, right: &Expr) -> Self::Output;
    fn visit_grouping_expr(&mut self, expression: &Expr) -> Self::Output;
    fn visit_literal_expr(&mut self, value: &LiteralType) -> Self::Output;
    fn visit_unary_expr(&mut self, operator: &Token, right: &Expr) -> Self::Output;
}

pub fn walk_expr<V: Visitor>(visitor: &mut V, expr: &Expr) -> V::Output {
    match expr {
        Expr::Binary {
            left,
            operator,
            right,
        } => visitor.visit_binary_expr(left, operator, right),
        Expr::Grouping { expression } => visitor.visit_grouping_expr(expression),
        Expr::Literal { value } => visitor.visit_literal_expr(value),
        Expr::Unary { operator, right } => visitor.visit_unary_expr(operator, right),
    }
}

pub struct AstPrinter;

impl AstPrinter {
    fn parenthesize(&mut self, name: &str, expressions: &[&Expr]) -> String {
        let mut output = String::new();
        output.push('(');
        output.push_str(name);

        for expr in expressions {
            output.push(' ');
            output.push_str(&walk_expr(self, expr));
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
            LiteralType::StringLiteral(s) => s.to_string(),
            LiteralType::NumberLiteral(n) => n.to_string(),
        }
    }

    fn visit_unary_expr(&mut self, operator: &Token, right: &Expr) -> String {
        self.parenthesize(&operator.lexeme, &[right])
    }
}
