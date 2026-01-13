use vstd::prelude::*;

verus! {

pub open spec fn option_bind<A, B>(m: Option<A>, f: spec_fn(A) -> Option<B>) -> Option<B> {
    match m {
        None => None,
        Some(a) => f(a),
    }
}

pub open spec fn option_return<A>(a: A) -> Option<A> {
    Some(a)
}


pub proof fn option_left_identity<A, B>(a: A, f: spec_fn(A) -> Option<B>)
    ensures option_bind(option_return(a), f) == f(a)
{
}

} // verus!