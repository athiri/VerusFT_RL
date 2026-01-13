use vstd::prelude::*;

verus! {

pub proof fn lemma_add_0_l(n: nat)
    ensures add(0, n) == n
{
}


pub proof fn lemma_mul_0_r(n: nat)
    ensures mul(n, 0) == 0
    decreases n
{
    if n == 0 {
    } else {
        lemma_mul_0_r((n - 1) as nat);
        // mul(n,0) = add(0, mul(n-1,0))
        // add(0,x)=x
        lemma_add_0_l(mul((n - 1) as nat, 0));
    }
}

} // verus!