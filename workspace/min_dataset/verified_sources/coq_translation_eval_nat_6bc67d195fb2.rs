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
    Eq { e1: Box<Expr>, e2: Box<Expr> },
    Lt { e1: Box<Expr>, e2: Box<Expr> },
    Add { e1: Box<Expr>, e2: Box<Expr> },
}


pub open spec fn eval_nat(e: Expr) -> Option<nat>
    decreases e
{
    match e {
        Expr::Zero => Option::Some(0),
        Expr::Succ { e } => match eval_nat(*e) {
            Option::Some(n) => Option::Some(n + 1),
            Option::None => Option::None,
        },
        Expr::Pred { e } => match eval_nat(*e) {
            Option::Some(n) => if n > 0 { Option::Some((n - 1) as nat) } else { Option::Some(0) },
            Option::None => Option::None,
        },
        _ => Option::None,
    }
}

} // verus!