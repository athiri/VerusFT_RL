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
    Add { e1: Box<Expr>, e2: Box<Expr> },
    Not { e: Box<Expr> },
}


pub open spec fn shrink_aggressive(e: Expr) -> Seq<Expr>
    decreases e
{
    match e {
        Expr::Var { .. } => seq![],
        Expr::Tru => seq![],
        Expr::Fls => seq![],
        Expr::Zero => seq![],
        Expr::Succ { e: inner } => seq![Expr::Zero, *inner],
        Expr::Pred { e: inner } => seq![Expr::Zero, *inner],
        Expr::IsZero { e: inner } => seq![Expr::Tru, Expr::Fls, *inner],
        Expr::App { e1, e2 } => seq![*e1, *e2],
        Expr::Add { e1, e2 } => seq![Expr::Zero, *e1, *e2],
        Expr::If { cond, then_br, else_br } => seq![*cond, *then_br, *else_br],
        Expr::Lam { body, .. } => seq![*body],
        Expr::Not { e: inner } => seq![*inner],
    }
}

} // verus!