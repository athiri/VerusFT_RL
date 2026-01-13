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

pub open spec fn eval(e: Expr) -> nat decreases e {
    match e {
        Expr::Const(n) => n,
        Expr::Add { left, right } => eval(*left) + eval(*right),
        Expr::Mul { left, right } => eval(*left) * eval(*right),
    }
}


pub proof fn const_fold_preserves_eval(e: Expr) ensures eval(const_fold(e)) == eval(e) decreases e {
    reveal_with_fuel(eval, 2); reveal_with_fuel(const_fold, 2);
    match e { Expr::Const(_) => {} Expr::Add { left, right } => { const_fold_preserves_eval(*left); const_fold_preserves_eval(*right); } Expr::Mul { left, right } => { const_fold_preserves_eval(*left); const_fold_preserves_eval(*right); } }
}

} // verus!