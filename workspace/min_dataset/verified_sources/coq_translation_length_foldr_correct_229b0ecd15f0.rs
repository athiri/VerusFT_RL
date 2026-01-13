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


pub open spec fn length_foldr<A>(xs: Seq<A>) -> nat {
    foldr(xs, 0nat, |_a: A, acc: nat| (acc + 1) as nat)
}


pub proof fn length_foldr_correct<A>(xs: Seq<A>)
    ensures length_foldr(xs) == xs.len()
    decreases xs.len()
{
    if xs.len() == 0 {
        assert(length_foldr(xs) == 0);
    } else {
        length_foldr_correct(xs.skip(1));
        assert(length_foldr(xs) == 1 + length_foldr(xs.skip(1)));
        assert(xs.skip(1).len() == xs.len() - 1);
    }
}

} // verus!