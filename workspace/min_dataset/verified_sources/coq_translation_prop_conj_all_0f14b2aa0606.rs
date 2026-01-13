use vstd::prelude::*;

verus! {

pub open spec fn prop_conj_all(props: Seq<bool>) -> bool
    decreases props.len()
{
    if props.len() == 0 {
        true
    } else {
        props[0] && prop_conj_all(props.skip(1))
    }
}

} // verus!