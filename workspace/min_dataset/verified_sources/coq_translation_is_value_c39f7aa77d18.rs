use vstd::prelude::*;

verus! {

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

pub open spec fn is_numeric_value(e: Expr) -> bool
    decreases e
{
    match e {
        Expr::Zero => true,
        Expr::Succ { e } => is_numeric_value(*e),
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

} // verus!