use vstd::prelude::*;

verus! {

pub open spec fn store_get(st: Store, x: Var) -> int {
    if st.dom().contains(x) { st[x] } else { 0 }
}


pub type Var = nat;

pub enum AExp {
    ANum { n: int },
    AId { x: Var },
    APlus { a1: Box<AExp>, a2: Box<AExp> },
    AMinus { a1: Box<AExp>, a2: Box<AExp> },
    AMult { a1: Box<AExp>, a2: Box<AExp> },
}

pub type Store = Map<Var, int>;


pub open spec fn aeval(a: AExp, st: Store) -> int
    decreases a
{
    match a {
        AExp::ANum { n } => n,
        AExp::AId { x } => store_get(st, x),
        AExp::APlus { a1, a2 } => aeval(*a1, st) + aeval(*a2, st),
        AExp::AMinus { a1, a2 } => aeval(*a1, st) - aeval(*a2, st),
        AExp::AMult { a1, a2 } => aeval(*a1, st) * aeval(*a2, st),
    }
}

} // verus!