use vstd::prelude::*;

verus! {

pub open spec fn option_map<A, B>(o: Option<A>, f: spec_fn(A) -> B) -> Option<B> {
    match o {
        Option::None => Option::None,
        Option::Some(x) => Option::Some(f(x)),
    }
}


pub proof fn ex8_option_map_none<A, B>(f: spec_fn(A) -> B)
    ensures option_map(Option::<A>::None, f) == Option::<B>::None
{
    assert(option_map(Option::<A>::None, f) == Option::<B>::None);
}

} // verus!