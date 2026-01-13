use vstd::prelude::*;

verus! {

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


pub open spec fn expr_size(e: Expr) -> nat
    decreases e
{
    match e {
        Expr::Var { .. } => 1,
        Expr::Tru => 1,
        Expr::Fls => 1,
        Expr::If { cond, then_br, else_br } =>
            1 + expr_size(*cond) + expr_size(*then_br) + expr_size(*else_br),
        Expr::Zero => 1,
        Expr::Succ { e } => 1 + expr_size(*e),
        Expr::Pred { e } => 1 + expr_size(*e),
        Expr::IsZero { e } => 1 + expr_size(*e),
        Expr::And { e1, e2 } => 1 + expr_size(*e1) + expr_size(*e2),
        Expr::Or { e1, e2 } => 1 + expr_size(*e1) + expr_size(*e2),
        Expr::Not { e } => 1 + expr_size(*e),
        Expr::Add { e1, e2 } => 1 + expr_size(*e1) + expr_size(*e2),
        Expr::Mul { e1, e2 } => 1 + expr_size(*e1) + expr_size(*e2),
    }
}

} // verus!