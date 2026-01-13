use vstd::prelude::*;

verus! {

pub enum Expr {
    Const(nat),
    Add { left: Box<Expr>, right: Box<Expr> },
    Mul { left: Box<Expr>, right: Box<Expr> },
}


pub open spec fn const_fold(e: Expr) -> Expr decreases e {
    match e {
        Expr::Const(n) => Expr::Const(n),
        Expr::Add { left, right } => {
            let l = const_fold(*left);
            let r = const_fold(*right);
            match (l, r) {
                (Expr::Const(a), Expr::Const(b)) => Expr::Const(a + b),
                _ => Expr::Add { left: Box::new(l), right: Box::new(r) }
            }
        }
        Expr::Mul { left, right } => {
            let l = const_fold(*left);
            let r = const_fold(*right);
            match (l, r) {
                (Expr::Const(a), Expr::Const(b)) => Expr::Const(a * b),
                _ => Expr::Mul { left: Box::new(l), right: Box::new(r) }
            }
        }
    }
}

} // verus!