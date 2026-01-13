use vstd::prelude::*;

verus! {

pub struct ClassifiedProp<C> {
    pub category: C,
    pub passed: bool,
    pub value: int,  // The test value (for statistics)
}


pub open spec fn count_classified_passing_helper<C>(props: Seq<ClassifiedProp<C>>, idx: int) -> nat
    decreases props.len() - idx
{
    if idx >= props.len() {
        0
    } else {
        let count = if props[idx].passed { 1 as nat } else { 0 as nat };
        count + count_classified_passing_helper(props, idx + 1)
    }
}

} // verus!