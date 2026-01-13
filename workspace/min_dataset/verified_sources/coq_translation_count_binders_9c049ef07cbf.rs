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
    Let { x: Var, def: Box<Expr>, body: Box<Expr> },
}


pub open spec fn count_binders(e: Expr) -> nat
    decreases e
{
    match e {
        Expr::Var { .. } => 0,
        Expr::Lam { body, .. } => 1 + count_binders(*body),
        Expr::App { e1, e2 } => count_binders(*e1) + count_binders(*e2),
        Expr::Tru => 0,
        Expr::Fls => 0,
        Expr::If { cond, then_br, else_br } =>
            count_binders(*cond) + count_binders(*then_br) + count_binders(*else_br),
        Expr::Zero => 0,
        Expr::Succ { e } => count_binders(*e),
        Expr::Pred { e } => count_binders(*e),
        Expr::IsZero { e } => count_binders(*e),
        Expr::Let { def, body, .. } => 1 + count_binders(*def) + count_binders(*body),
    }
}

} // verus!