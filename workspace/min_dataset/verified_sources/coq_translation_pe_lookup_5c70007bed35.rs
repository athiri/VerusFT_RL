use vstd::prelude::*;

verus! {

pub type Var = nat;

pub type PEState = Seq<(Var, int)>;


pub open spec fn pe_lookup(st: PEState, x: Var) -> Option<int>
    decreases st.len()
{
    if st.len() == 0 {
        Option::None
    } else {
        let (v, val) = st[0];
        if v == x {
            Option::Some(val)
        } else {
            pe_lookup(st.skip(1), x)
        }
    }
}

} // verus!