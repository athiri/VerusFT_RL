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


pub open spec fn expr_depth(e: Expr) -> nat
    decreases e
{
    match e {
        Expr::Var { .. } => 0,
        Expr::Lam { body, .. } => 1 + expr_depth(*body),
        Expr::App { e1, e2 } => {
            let d1 = expr_depth(*e1);
            let d2 = expr_depth(*e2);
            1 + if d1 > d2 { d1 } else { d2 }
        }
        Expr::Tru => 0,
        Expr::Fls => 0,
        Expr::If { cond, then_br, else_br } => {
            let d1 = expr_depth(*cond);
            let d2 = expr_depth(*then_br);
            let d3 = expr_depth(*else_br);
            let max12 = if d1 > d2 { d1 } else { d2 };
            1 + if max12 > d3 { max12 } else { d3 }
        }
        Expr::Zero => 0,
        Expr::Succ { e } => 1 + expr_depth(*e),
        Expr::Pred { e } => 1 + expr_depth(*e),
        Expr::IsZero { e } => 1 + expr_depth(*e),
        Expr::Let { def, body, .. } => {
            let d1 = expr_depth(*def);
            let d2 = expr_depth(*body);
            1 + if d1 > d2 { d1 } else { d2 }
        }
    }
}

} // verus!