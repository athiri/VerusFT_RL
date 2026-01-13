use vstd::prelude::*;

verus! {

pub open spec fn option_bind<A, B>(m: Option<A>, f: spec_fn(A) -> Option<B>) -> Option<B> {
    match m {
        None => None,
        Some(a) => f(a),
    }
}


pub proof fn option_associativity<A, B, C>(
    m: Option<A>,
    f: spec_fn(A) -> Option<B>,
    g: spec_fn(B) -> Option<C>
)
    ensures option_bind(option_bind(m, f), g) ==
            option_bind(m, |x: A| option_bind(f(x), g))
{
}

} // verus!