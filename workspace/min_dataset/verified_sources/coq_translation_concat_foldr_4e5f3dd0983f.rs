use vstd::prelude::*;

verus! {

pub open spec fn foldr<A, B>(xs: Seq<A>, init: B, f: spec_fn(A, B) -> B) -> B
    decreases xs.len()
{
    if xs.len() == 0 {
        init
    } else {
        f(xs[0], foldr(xs.skip(1), init, f))
    }
}


pub open spec fn concat_foldr<A>(xss: Seq<Seq<A>>) -> Seq<A>
    decreases xss.len()
{
    foldr(xss, Seq::empty(), |xs: Seq<A>, acc: Seq<A>| xs.add(acc))
}

} // verus!