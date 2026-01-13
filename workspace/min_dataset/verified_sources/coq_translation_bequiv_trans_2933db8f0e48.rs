use vstd::prelude::*;

verus! {

pub open spec fn store_get(st: Store, x: Var) -> int {
    if st.dom().contains(x) { st[x] } else { 0 }
}


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


pub open spec fn beval(b: BExp, st: Store) -> bool
    decreases b
{
    match b {
        BExp::BTrue => true,
        BExp::BFalse => false,
        BExp::BEq { a1, a2 } => aeval(*a1, st) == aeval(*a2, st),
        BExp::BLe { a1, a2 } => aeval(*a1, st) <= aeval(*a2, st),
        BExp::BNot { b1 } => !beval(*b1, st),
        BExp::BAnd { b1, b2 } => beval(*b1, st) && beval(*b2, st),
    }
}


pub type Var = nat;

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
    BEq { a1: Box<AExp>, a2: Box<AExp> },
    BLe { a1: Box<AExp>, a2: Box<AExp> },
    BNot { b1: Box<BExp> },
    BAnd { b1: Box<BExp>, b2: Box<BExp> },
}

pub type Store = Map<Var, int>;

pub open spec fn bequiv(b1: BExp, b2: BExp) -> bool {
    forall|st: Store| beval(b1, st) == beval(b2, st)
}


pub proof fn bequiv_trans(b1: BExp, b2: BExp, b3: BExp)
    requires
        bequiv(b1, b2),
        bequiv(b2, b3),
    ensures bequiv(b1, b3)
{
}

} // verus!