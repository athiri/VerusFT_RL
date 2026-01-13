use vstd::prelude::*;

verus! {

pub open spec fn is_succ_expr(e: Expr) -> bool {
    match e {
        Expr::Succ { .. } => true,
        _ => false,
    }
}

pub open spec fn is_zero_expr(e: Expr) -> bool {
    match e {
        Expr::Zero => true,
        _ => false,
    }
}


pub type Var = nat;

pub enum Expr {
    Var { x: Var },
    Tru,
    Fls,
    If { cond: Box<Expr>, then_br: Box<Expr>, else_br: Box<Expr> },
    Zero,
    Succ { e: Box<Expr> },
    Pred { e: Box<Expr> },
    IsZero { e: Box<Expr> },
    And { e1: Box<Expr>, e2: Box<Expr> },
    Or { e1: Box<Expr>, e2: Box<Expr> },
    Not { e: Box<Expr> },
    Add { e1: Box<Expr>, e2: Box<Expr> },
    Mul { e1: Box<Expr>, e2: Box<Expr> },
}


pub open spec fn is_nat_canonical(e: Expr) -> bool
    decreases e
{
    match e {
        Expr::Zero => true,
        Expr::Var { .. } => true,
        Expr::Succ { e } => is_nat_canonical(*e),
        Expr::Add { e1, e2 } =>
            !is_zero_expr(*e1) && !is_zero_expr(*e2) &&
            is_nat_canonical(*e1) && is_nat_canonical(*e2),
        Expr::Mul { e1, e2 } =>
            !is_zero_expr(*e1) && !is_zero_expr(*e2) &&
            is_nat_canonical(*e1) && is_nat_canonical(*e2),
        Expr::Pred { e } =>
            !is_zero_expr(*e) && !is_succ_expr(*e) && is_nat_canonical(*e),
        _ => false,
    }
}

} // verus!