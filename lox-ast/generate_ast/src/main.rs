use tokens::types::TokenType;

pub enum Expr {
    Binary(BinaryExpr),
    Grouping(GroupingExpr),
    Literal(LiteralExpr),
    Unary(UnaryExpr),
}

pub struct BinaryExpr {
    left: Box<Expr>,
    operator: TokenType,
    right: Box<Expr>,
}
pub struct GroupingExpr {
    expression: Box<Expr>,
}

pub struct LiteralExpr {
    value: Object,
}
pub struct UnaryExpr {
    operator: TokenType,
    right: Box<Expr>,
}

fn main() {
    println!("Hello, world!");
}
