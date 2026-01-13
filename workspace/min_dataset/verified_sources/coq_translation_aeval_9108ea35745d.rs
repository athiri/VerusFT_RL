use vstd::prelude::*;

verus! {

pub open spec fn store_get(st: Store, default: int, x: Var) -> int {
    if st.dom().contains(x) { st[x] } else { default }
}


pub type Var = nat;

pub enum AExp {
    N { n: int },
    V { x: Var },
    Plus { a1: Box<AExp>, a2: Box<AExp> },
    Minus { a1: Box<AExp>, a2: Box<AExp> },
}

pub type Store = Map<Var, int>;


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

} // verus!