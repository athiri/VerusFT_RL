use vstd::prelude::*;

verus! {

pub open spec fn option_map<A, B>(o: Option<A>, f: spec_fn(A) -> B) -> Option<B> {
    match o {
        Option::None => Option::None,
        Option::Some(x) => Option::Some(f(x)),
    }
}


pub proof fn ex10_option_map_comp<A, B, C>(o: Option<A>, f: spec_fn(A) -> B, g: spec_fn(B) -> C)
    ensures option_map(option_map(o, f), g) == option_map(o, |a: A| g(f(a)))
{
    match o {
        Option::None => {
            assert(option_map(option_map(o, f), g) == Option::<C>::None);
            assert(option_map(o, |a: A| g(f(a))) == Option::<C>::None);
        }
        Option::Some(x) => {
            assert(option_map(option_map(o, f), g) == Option::Some(g(f(x))));
            assert(option_map(o, |a: A| g(f(a))) == Option::Some(g(f(x))));
        }
    }
}

} // verus!