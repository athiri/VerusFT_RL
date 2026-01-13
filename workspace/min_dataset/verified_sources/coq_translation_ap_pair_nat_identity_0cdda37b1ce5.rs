use vstd::prelude::*;

verus! {

pub open spec fn ap_pair_nat<A, B>(
    pf: (nat, spec_fn(A) -> B),
    pa: (nat, A)
) -> (nat, B) {
    ((pf.0 + pa.0) as nat, pf.1(pa.1))
}


pub proof fn ap_pair_nat_identity<A>(v: (nat, A))
    ensures ap_pair_nat((0, |a: A| a), v) == v
{
    let result = ap_pair_nat((0, |a: A| a), v);
    assert(result.0 == (0 + v.0) as nat);
    assert(result.0 == v.0);
    assert(result.1 == v.1);
}

} // verus!