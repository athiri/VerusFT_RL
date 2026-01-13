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


pub open spec fn nat_to_expr(n: nat) -> Expr
    decreases n
{
    if n == 0 {
        Expr::Zero
    } else {
        Expr::Succ { e: Box::new(nat_to_expr((n - 1) as nat)) }
    }
}

} // verus!