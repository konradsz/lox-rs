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
