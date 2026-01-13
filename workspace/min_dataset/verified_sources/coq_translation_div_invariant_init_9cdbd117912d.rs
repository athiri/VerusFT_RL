use vstd::prelude::*;

verus! {

pub type Assertion = spec_fn(State) -> bool;

pub spec const Y: Var = 1;

pub open spec fn div_invariant(m: int) -> Assertion {
    |st: State| lookup(st, Q) * lookup(st, Y) + lookup(st, R) == m
}


pub type Var = nat;

pub type State = Map<Var, int>;

pub open spec fn lookup(st: State, x: Var) -> int {
    if st.dom().contains(x) { st[x] } else { 0 }
}

pub spec const Q: Var = 3;

pub spec const R: Var = 4;


pub proof fn div_invariant_init(m: int)
    ensures forall|st: State|
        lookup(st, Q) == 0 && lookup(st, R) == m ==>
        (div_invariant(m))(st)
{
}

} // verus!