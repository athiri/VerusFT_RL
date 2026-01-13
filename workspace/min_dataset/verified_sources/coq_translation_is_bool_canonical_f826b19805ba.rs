use vstd::prelude::*;

verus! {

pub open spec fn is_not_expr(e: Expr) -> bool {
    match e {
        Expr::Not { .. } => true,
        _ => false,
    }
}


pub open spec fn is_tru_expr(e: Expr) -> bool {
    match e {
        Expr::Tru => true,
        _ => false,
    }
}

pub open spec fn is_fls_expr(e: Expr) -> bool {
    match e {
        Expr::Fls => true,
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


pub open spec fn is_bool_canonical(e: Expr) -> bool
    decreases e
{
    match e {
        Expr::Tru => true,
        Expr::Fls => true,
        Expr::Var { .. } => true,
        Expr::And { e1, e2 } =>
            !is_tru_expr(*e1) && !is_fls_expr(*e1) && !is_tru_expr(*e2) && !is_fls_expr(*e2) &&
            is_bool_canonical(*e1) && is_bool_canonical(*e2),
        Expr::Or { e1, e2 } =>
            !is_tru_expr(*e1) && !is_fls_expr(*e1) && !is_tru_expr(*e2) && !is_fls_expr(*e2) &&
            is_bool_canonical(*e1) && is_bool_canonical(*e2),
        Expr::Not { e } =>
            !is_tru_expr(*e) && !is_fls_expr(*e) && !is_not_expr(*e) && is_bool_canonical(*e),
        _ => false,
    }
}

} // verus!