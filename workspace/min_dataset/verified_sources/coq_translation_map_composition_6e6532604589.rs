use vstd::prelude::*;

verus! {

pub open spec fn gen_map<A, B>(gen: Set<A>, f: spec_fn(A) -> B) -> Set<B> {
    Set::new(|b: B| exists|a: A| gen.contains(a) && f(a) == b)
}


pub proof fn map_composition<A, B, C>(gen: Set<A>, f: spec_fn(B) -> C, g: spec_fn(A) -> B, a: A, b: B, c: C)
    requires gen.contains(a), g(a) == b, f(b) == c
    ensures gen_map(gen, |x: A| f(g(x))).contains(c)
{
}

} // verus!