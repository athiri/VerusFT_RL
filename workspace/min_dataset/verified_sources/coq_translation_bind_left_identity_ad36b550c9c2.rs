use vstd::prelude::*;

verus! {

pub open spec fn gen_bind<A, B>(gen_a: Set<A>, f: spec_fn(A) -> Set<B>) -> Set<B> {
    Set::new(|b: B| exists|a: A| gen_a.contains(a) && f(a).contains(b))
}

pub open spec fn gen_return<A>(a: A) -> Set<A> {
    Set::new(|x: A| x == a)
}


pub proof fn bind_left_identity<A, B>(a: A, f: spec_fn(A) -> Set<B>, b: B)
    requires f(a).contains(b)
    ensures gen_bind(gen_return(a), f).contains(b)
{
    // Witness that a is in gen_return(a) and f(a) contains b
    assert(gen_return(a).contains(a));
    assert(f(a).contains(b));
}

} // verus!