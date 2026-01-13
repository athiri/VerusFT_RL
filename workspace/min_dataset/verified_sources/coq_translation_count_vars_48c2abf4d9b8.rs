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


pub open spec fn count_vars(e: Expr) -> nat
    decreases e
{
    match e {
        Expr::Var { .. } => 1,
        Expr::Lam { body, .. } => count_vars(*body),
        Expr::App { e1, e2 } => count_vars(*e1) + count_vars(*e2),
        Expr::Tru => 0,
        Expr::Fls => 0,
        Expr::If { cond, then_br, else_br } =>
            count_vars(*cond) + count_vars(*then_br) + count_vars(*else_br),
        Expr::Zero => 0,
        Expr::Succ { e } => count_vars(*e),
        Expr::Pred { e } => count_vars(*e),
        Expr::IsZero { e } => count_vars(*e),
        Expr::Let { def, body, .. } => count_vars(*def) + count_vars(*body),
    }
}

} // verus!