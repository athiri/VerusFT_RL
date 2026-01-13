use vstd::prelude::*;

verus! {

pub open spec fn prop_disj_any(props: Seq<bool>) -> bool
    decreases props.len()
{
    if props.len() == 0 {
        false
    } else {
        props[0] || prop_disj_any(props.skip(1))
    }
}

} // verus!