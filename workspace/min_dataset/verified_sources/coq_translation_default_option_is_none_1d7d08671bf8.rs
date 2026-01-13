use vstd::prelude::*;

verus! {

pub open spec fn default_option<A>() -> Option<A> {
    Option::None
}


pub proof fn default_option_is_none<A>()
    ensures default_option::<A>() == Option::<A>::None
{
    // Trivially true by definition
}

} // verus!