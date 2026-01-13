use vstd::prelude::*;

verus! {

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

pub open spec fn expr_size(e: Expr) -> nat
    decreases e
{
    match e {
        Expr::Var { .. } => 1,
        Expr::Lam { body, .. } => 1 + expr_size(*body),
        Expr::App { e1, e2 } => 1 + expr_size(*e1) + expr_size(*e2),
        Expr::Tru => 1,
        Expr::Fls => 1,
        Expr::If { cond, then_br, else_br } =>
            1 + expr_size(*cond) + expr_size(*then_br) + expr_size(*else_br),
        Expr::Zero => 1,
        Expr::Succ { e } => 1 + expr_size(*e),
        Expr::Pred { e } => 1 + expr_size(*e),
        Expr::IsZero { e } => 1 + expr_size(*e),
        Expr::Let { def, body, .. } => 1 + expr_size(*def) + expr_size(*body),
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
    Let { x: Var, def: Box<Expr>, body: Box<Expr> },
}


pub proof fn binders_bounded_by_size(e: Expr)
    ensures count_binders(e) < expr_size(e)
    decreases e
{
    match e {
        Expr::Var { .. } => {}
        Expr::Lam { body, .. } => binders_bounded_by_size(*body),
        Expr::App { e1, e2 } => {
            binders_bounded_by_size(*e1);
            binders_bounded_by_size(*e2);
        }
        Expr::Tru => {}
        Expr::Fls => {}
        Expr::If { cond, then_br, else_br } => {
            binders_bounded_by_size(*cond);
            binders_bounded_by_size(*then_br);
            binders_bounded_by_size(*else_br);
        }
        Expr::Zero => {}
        Expr::Succ { e } => binders_bounded_by_size(*e),
        Expr::Pred { e } => binders_bounded_by_size(*e),
        Expr::IsZero { e } => binders_bounded_by_size(*e),
        Expr::Let { def, body, .. } => {
            binders_bounded_by_size(*def);
            binders_bounded_by_size(*body);
        }
    }
}

} // verus!