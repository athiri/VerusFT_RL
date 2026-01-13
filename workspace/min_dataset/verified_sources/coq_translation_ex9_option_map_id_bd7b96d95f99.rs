use vstd::prelude::*;

verus! {

pub open spec fn option_map<A, B>(o: Option<A>, f: spec_fn(A) -> B) -> Option<B> {
    match o {
        Option::None => Option::None,
        Option::Some(x) => Option::Some(f(x)),
    }
}


pub proof fn ex9_option_map_id<A>(o: Option<A>)
    ensures option_map(o, |a: A| a) == o
{
    match o {
        Option::None => {
            assert(option_map(o, |a: A| a) == Option::<A>::None);
        }
        Option::Some(x) => {
            assert(option_map(o, |a: A| a) == Option::Some(x));
        }
    }
}

} // verus!