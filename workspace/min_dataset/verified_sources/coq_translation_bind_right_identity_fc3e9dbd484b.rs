use vstd::prelude::*;

verus! {

pub open spec fn gen_bind<A, B>(gen_a: Set<A>, f: spec_fn(A) -> Set<B>) -> Set<B> {
    Set::new(|b: B| exists|a: A| gen_a.contains(a) && f(a).contains(b))
}

pub open spec fn gen_return<A>(a: A) -> Set<A> {
    Set::new(|x: A| x == a)
}


pub proof fn bind_right_identity<A>(gen: Set<A>, a: A)
    requires gen.contains(a)
    ensures gen_bind(gen, |x: A| gen_return(x)).contains(a)
{
}

} // verus!