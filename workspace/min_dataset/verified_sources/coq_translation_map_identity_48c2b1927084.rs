use vstd::prelude::*;

verus! {

pub open spec fn gen_map<A, B>(gen: Set<A>, f: spec_fn(A) -> B) -> Set<B> {
    Set::new(|b: B| exists|a: A| gen.contains(a) && f(a) == b)
}


pub proof fn map_identity<A>(gen: Set<A>, a: A)
    requires gen.contains(a)
    ensures gen_map(gen, |x: A| x).contains(a)
{
}

} // verus!