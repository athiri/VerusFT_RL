use vstd::prelude::*;

verus! {

pub open spec fn is_numeric_value(e: Expr) -> bool
    decreases e
{
    match e {
        Expr::Zero => true,
        Expr::Succ { e } => is_numeric_value(*e),
        _ => false,
    }
}

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


pub open spec fn is_value(e: Expr) -> bool
    decreases e
{
    match e {
        Expr::Lam { .. } => true,
        Expr::Tru => true,
        Expr::Fls => true,
        Expr::Zero => true,
        Expr::Succ { e } => is_numeric_value(*e),
        _ => false,
    }
}

pub open spec fn is_lam_expr(e: Expr) -> bool {
    match e {
        Expr::Lam { .. } => true,
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

pub enum Ty {
    TBool,
    TNat,
    TArrow { t1: Box<Ty>, t2: Box<Ty> },
}

pub enum Expr {
    Var { x: Var },
    Lam { x: Var, ty: Ty, body: Box<Expr> },
    App { e1: Box<Expr>, e2: Box<Expr> },
    Tru,
    Fls,
    If { cond: Box<Expr>, then_br: Box<Expr>, else_br: Box<Expr> },
    Zero,
    Succ { e: Box<Expr> },
    Pred { e: Box<Expr> },
    IsZero { e: Box<Expr> },
}


pub open spec fn can_step(e: Expr) -> bool
    decreases e
{
    match e {
        Expr::App { e1, e2 } => {
            can_step(*e1) ||
            (is_value(*e1) && can_step(*e2)) ||
            (is_value(*e1) && is_value(*e2) && is_lam_expr(*e1))
        }
        Expr::If { cond, .. } => {
            can_step(*cond) || is_tru_expr(*cond) || is_fls_expr(*cond)
        }
        Expr::Succ { e } => can_step(*e),
        Expr::Pred { e } => can_step(*e) || is_zero_expr(*e) || (is_succ_expr(*e) && is_numeric_value(*e)),
        Expr::IsZero { e } => can_step(*e) || is_zero_expr(*e) || (is_succ_expr(*e) && is_numeric_value(*e)),
        _ => false,
    }
}

} // verus!