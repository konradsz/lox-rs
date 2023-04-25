use crate::token::Token;

pub trait Visitor {
    fn visit_binary_expr(&mut self, left: &Expr, operator: &Token, right: &Expr);
    fn visit_grouping_expr(&mut self, expression: &Expr);
    fn visit_literal_expr(&mut self, value: &LiteralType);
    fn visit_unary_expr(&mut self, operator: &Token, right: &Expr);
}
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: LiteralType,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
}

pub enum LiteralType {
    StringLiteral(String),
    NumberLiteral(f64),
}

pub fn walk_expr(visitor: &mut dyn Visitor, expr: &Expr) {
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

#[derive(Default)]
pub struct AstPrinter {
    pub output: String,
}

impl AstPrinter {
    fn parenthesize(&mut self, name: &str, expressions: &[&Expr]) {
        self.output.push('(');
        self.output.push_str(name);

        for expr in expressions {
            self.output.push(' ');
            walk_expr(self, expr);
        }

        self.output.push(')');
    }
}

impl Visitor for AstPrinter {
    fn visit_binary_expr(&mut self, left: &Expr, operator: &Token, right: &Expr) {
        self.parenthesize(&operator.lexeme, &[left, right]);
    }

    fn visit_grouping_expr(&mut self, expression: &Expr) {
        self.parenthesize("group", &[expression]);
    }

    fn visit_literal_expr(&mut self, value: &LiteralType) {
        match value {
            LiteralType::StringLiteral(s) => self.output.push_str(s),
            LiteralType::NumberLiteral(n) => self.output.push_str(&n.to_string()),
        }
    }

    fn visit_unary_expr(&mut self, operator: &Token, right: &Expr) {
        self.parenthesize(&operator.lexeme, &[right]);
    }
}
