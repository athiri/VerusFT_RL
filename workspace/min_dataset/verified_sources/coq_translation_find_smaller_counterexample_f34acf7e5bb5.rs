use vstd::prelude::*;

verus! {

pub open spec fn is_counterexample(e: Expr, prop: spec_fn(Expr) -> bool) -> bool {
    !prop(e)
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


pub open spec fn find_smaller_counterexample(shrinks: Seq<Expr>, prop: spec_fn(Expr) -> bool, default: Expr) -> Expr
    decreases shrinks.len()
{
    if shrinks.len() == 0 {
        default  // No smaller counterexample found
    } else {
        let first = shrinks[0];
        if is_counterexample(first, prop) {
            first  // Found a smaller counterexample
        } else {
            find_smaller_counterexample(shrinks.drop_first(), prop, default)
        }
    }
}

} // verus!