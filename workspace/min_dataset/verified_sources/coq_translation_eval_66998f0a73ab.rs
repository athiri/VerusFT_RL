use vstd::prelude::*;

verus! {

pub enum Expr {
    Const(nat),
    Add { left: Box<Expr>, right: Box<Expr> },
    Mul { left: Box<Expr>, right: Box<Expr> },
}


pub open spec fn eval(e: Expr) -> nat decreases e {
    match e {
        Expr::Const(n) => n,
        Expr::Add { left, right } => eval(*left) + eval(*right),
        Expr::Mul { left, right } => eval(*left) * eval(*right),
    }
}

} // verus!