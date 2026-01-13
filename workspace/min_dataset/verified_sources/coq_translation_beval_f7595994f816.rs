use vstd::prelude::*;

verus! {

pub open spec fn store_get(st: Store, default: int, x: Var) -> int {
    if st.dom().contains(x) { st[x] } else { default }
}


pub open spec fn aeval(a: AExp, st: Store, default: int) -> int
    decreases a
{
    match a {
        AExp::N { n } => n,
        AExp::V { x } => store_get(st, default, x),
        AExp::Plus { a1, a2 } => aeval(*a1, st, default) + aeval(*a2, st, default),
        AExp::Minus { a1, a2 } => aeval(*a1, st, default) - aeval(*a2, st, default),
    }
}


pub type Var = nat;

pub enum AExp {
    N { n: int },
    V { x: Var },
    Plus { a1: Box<AExp>, a2: Box<AExp> },
    Minus { a1: Box<AExp>, a2: Box<AExp> },
}

pub enum BExp {
    B { b: bool },
    Eq { a1: Box<AExp>, a2: Box<AExp> },
    Le { a1: Box<AExp>, a2: Box<AExp> },
    Not { b1: Box<BExp> },
    And { b1: Box<BExp>, b2: Box<BExp> },
}

pub type Store = Map<Var, int>;


pub open spec fn beval(b: BExp, st: Store, default: int) -> bool
    decreases b
{
    match b {
        BExp::B { b } => b,
        BExp::Eq { a1, a2 } => aeval(*a1, st, default) == aeval(*a2, st, default),
        BExp::Le { a1, a2 } => aeval(*a1, st, default) <= aeval(*a2, st, default),
        BExp::Not { b1 } => !beval(*b1, st, default),
        BExp::And { b1, b2 } => beval(*b1, st, default) && beval(*b2, st, default),
    }
}

} // verus!