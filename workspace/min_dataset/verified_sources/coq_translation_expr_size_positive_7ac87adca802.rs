use vstd::prelude::*;

verus! {

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
        Expr::Add { e1, e2 } => 1 + expr_size(*e1) + expr_size(*e2),
        Expr::Not { e } => 1 + expr_size(*e),
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
    Add { e1: Box<Expr>, e2: Box<Expr> },
    Not { e: Box<Expr> },
}


pub proof fn expr_size_positive(e: Expr)
    ensures expr_size(e) >= 1
    decreases e
{
    match e {
        Expr::Var { .. } => {}
        Expr::Lam { body, .. } => expr_size_positive(*body),
        Expr::App { e1, e2 } => {
            expr_size_positive(*e1);
            expr_size_positive(*e2);
        }
        Expr::Tru => {}
        Expr::Fls => {}
        Expr::If { cond, then_br, else_br } => {
            expr_size_positive(*cond);
            expr_size_positive(*then_br);
            expr_size_positive(*else_br);
        }
        Expr::Zero => {}
        Expr::Succ { e } => expr_size_positive(*e),
        Expr::Pred { e } => expr_size_positive(*e),
        Expr::IsZero { e } => expr_size_positive(*e),
        Expr::Add { e1, e2 } => {
            expr_size_positive(*e1);
            expr_size_positive(*e2);
        }
        Expr::Not { e } => expr_size_positive(*e),
    }
}

} // verus!