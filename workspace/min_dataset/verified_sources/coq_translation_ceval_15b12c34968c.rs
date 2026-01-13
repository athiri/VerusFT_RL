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
    }
}

pub open spec fn update(st: State, x: Var, v: int) -> State {
    st.insert(x, v)
}


pub type Var = nat;

pub type State = Map<Var, int>;

pub enum AExp {
    ANum { n: int },
    AId { x: Var },
    APlus { a1: Box<AExp>, a2: Box<AExp> },
}

pub enum Com {
    CSkip,
    CAsgn { x: Var, a: AExp },
    CSeq { c1: Box<Com>, c2: Box<Com> },
}


pub open spec fn ceval(c: Com, st: State) -> State
    decreases c
{
    match c {
        Com::CSkip => st,
        Com::CAsgn { x, a } => update(st, x, aeval(st, a)),
        Com::CSeq { c1, c2 } => ceval(*c2, ceval(*c1, st)),
    }
}

} // verus!