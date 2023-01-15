pub mod nodes {
    use crate::tokens::Token;
    use std::convert::TryFrom;

    pub enum Expr {
        Literal(Literal),
        Unary(Token, Box<Expr>),
        Binary(Box<Expr>, Token, Box<Expr>),
        Grouping(Box<Expr>),
    }

    pub enum UnaryOperator {
        Bang,
        Minus,
    }

    impl TryFrom<String> for UnaryOperator {
        type Error = String;
        fn try_from(item: String) -> Result<Self, Self::Error> {
            match item.as_str() {
                "==" => Ok(UnaryOperator::Bang),
                "!=" => Ok(UnaryOperator::Minus),
                _ => Err(format!("Failed to decode [{}] as a Unary Operator", item).to_owned()),
            }
        }
    }

    pub enum BinaryOperator {
        EqualEqual,
        BangEqual,
        Less,
        LessEqual,
        Greater,
        GreaterEqual,
        Plus,
        Minus,
        Star,
        Slash,
    }

    impl TryFrom<String> for BinaryOperator {
        type Error = String;
        fn try_from(item: String) -> Result<Self, Self::Error> {
            match item.as_str() {
                "==" => Ok(BinaryOperator::EqualEqual),
                "!=" => Ok(BinaryOperator::BangEqual),
                "<" => Ok(BinaryOperator::Less),
                "<=" => Ok(BinaryOperator::LessEqual),
                ">" => Ok(BinaryOperator::Greater),
                ">=" => Ok(BinaryOperator::GreaterEqual),
                "+" => Ok(BinaryOperator::Plus),
                "-" => Ok(BinaryOperator::Minus),
                "*" => Ok(BinaryOperator::Star),
                "/" => Ok(BinaryOperator::Slash),
                _ => Err(format!("Failed to decode [{}] as a Binary Operator", item).to_owned()),
            }
        }
    }

    pub enum Literal {
        String(String),
        Number(f64),
        True,
        False,
        Nil,
    }
}

pub trait ExprVisitor {
    fn visit_expr(&self, expr: &nodes::Expr) -> String;
    fn visit_unary_operator(&self, operator: &nodes::UnaryOperator) -> String;
    fn visit_binary_operator(&self, operator: &nodes::BinaryOperator) -> String;
    fn visit_literal(&self, literal: &nodes::Literal) -> String;
}
pub trait Acceptor {
    fn accept(&self, visitor: &dyn ExprVisitor) -> String;
}

impl Acceptor for nodes::Expr {
    fn accept(&self, visitor: &dyn ExprVisitor) -> String {
        visitor.visit_expr(&self)
    }
}

impl Acceptor for nodes::UnaryOperator {
    fn accept(&self, visitor: &dyn ExprVisitor) -> String {
        visitor.visit_unary_operator(&self)
    }
}

impl Acceptor for nodes::BinaryOperator {
    fn accept(&self, visitor: &dyn ExprVisitor) -> String {
        visitor.visit_binary_operator(&self)
    }
}

impl Acceptor for nodes::Literal {
    fn accept(&self, visitor: &dyn ExprVisitor) -> String {
        visitor.visit_literal(&self)
    }
}

pub struct ASTPrinter {}

impl ASTPrinter {
    pub fn create() -> Self {
        ASTPrinter {}
    }
    pub fn print(&self, expr: &nodes::Expr) -> String {
        expr.accept(self)
    }
    fn parenthesize(&self, text: &String, exprs: &[&nodes::Expr]) -> String {
        let mut out = String::new();
        out.push('(');
        out += &text;

        for expr in exprs {
            out.push(' ');
            out += &expr.accept(self);
        }

        out.push(')');
        out
    }
}

impl ExprVisitor for ASTPrinter {
    fn visit_expr(&self, expr: &nodes::Expr) -> String {
        match expr {
            nodes::Expr::Literal(literal) => match literal {
                nodes::Literal::String(s) => s.clone(),
                nodes::Literal::Number(n) => n.to_string(),
                nodes::Literal::True => "True".to_owned(),
                nodes::Literal::False => "False".to_owned(),
                nodes::Literal::Nil => "Nil".to_owned(),
            },
            nodes::Expr::Unary(operator, expr) => self.parenthesize(&operator.lexeme, &[expr]),
            nodes::Expr::Binary(left, operator, right) => {
                self.parenthesize(&operator.lexeme, &[left, right])
            }
            nodes::Expr::Grouping(expr) => self.parenthesize(&"group".to_owned(), &[expr]),
        }
    }
    fn visit_unary_operator(&self, operator: &nodes::UnaryOperator) -> String {
        "".to_owned()
    }
    fn visit_binary_operator(&self, operator: &nodes::BinaryOperator) -> String {
        "".to_owned()
    }
    fn visit_literal(&self, literal: &nodes::Literal) -> String {
        "".to_owned()
    }
}
