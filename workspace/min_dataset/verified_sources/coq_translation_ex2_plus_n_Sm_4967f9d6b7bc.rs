use vstd::prelude::*;

verus! {

pub proof fn ex2_plus_n_Sm(n: nat, m: nat)
    ensures add(n, m + 1) == add(n, m) + 1
    decreases n
{
    if n == 0 {
        assert(add(0, m + 1) == m + 1);
        assert(add(0, m) + 1 == m + 1);
    } else {
        let n1 = (n - 1) as nat;
        ex2_plus_n_Sm(n1, m);
        assert(add(n, m + 1) == add(n1, m + 1) + 1);
        assert(add(n, m) == add(n1, m) + 1);
    }
}

} // verus!