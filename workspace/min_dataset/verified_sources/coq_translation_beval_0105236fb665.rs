use vstd::prelude::*;

verus! {

pub open spec fn lookup(st: State, x: Var) -> int {
    if st.dom().contains(x) { st[x] } else { 0 }
}


pub open spec fn aeval(st: State, a: AExp) -> int
    decreases a
{
    match a {
        AExp::ANum { n } => n,
        AExp::AId { x } => lookup(st, x),
        AExp::APlus { a1, a2 } => aeval(st, *a1) + aeval(st, *a2),
        AExp::AMinus { a1, a2 } => aeval(st, *a1) - aeval(st, *a2),
        AExp::AMult { a1, a2 } => aeval(st, *a1) * aeval(st, *a2),
    }
}


pub type Var = nat;

pub type State = Map<Var, int>;

pub enum AExp {
    ANum { n: int },
    AId { x: Var },
    APlus { a1: Box<AExp>, a2: Box<AExp> },
    AMinus { a1: Box<AExp>, a2: Box<AExp> },
    AMult { a1: Box<AExp>, a2: Box<AExp> },
}

pub enum BExp {
    BTrue,
    BFalse,
    BEq { a1: AExp, a2: AExp },
    BLe { a1: AExp, a2: AExp },
    BNot { b: Box<BExp> },
    BAnd { b1: Box<BExp>, b2: Box<BExp> },
}


pub open spec fn beval(st: State, b: BExp) -> bool
    decreases b
{
    match b {
        BExp::BTrue => true,
        BExp::BFalse => false,
        BExp::BEq { a1, a2 } => aeval(st, a1) == aeval(st, a2),
        BExp::BLe { a1, a2 } => aeval(st, a1) <= aeval(st, a2),
        BExp::BNot { b } => !beval(st, *b),
        BExp::BAnd { b1, b2 } => beval(st, *b1) && beval(st, *b2),
    }
}

} // verus!