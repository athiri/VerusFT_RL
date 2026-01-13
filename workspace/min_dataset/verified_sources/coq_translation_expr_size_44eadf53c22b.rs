use vstd::prelude::*;

verus! {

pub enum Expr {
    Const(nat),
    Add { left: Box<Expr>, right: Box<Expr> },
    Mul { left: Box<Expr>, right: Box<Expr> },
}


pub open spec fn expr_size(e: Expr) -> nat decreases e {
    match e {
        Expr::Const(_) => 1,
        Expr::Add { left, right } => 1 + expr_size(*left) + expr_size(*right),
        Expr::Mul { left, right } => 1 + expr_size(*left) + expr_size(*right),
    }
}

} // verus!