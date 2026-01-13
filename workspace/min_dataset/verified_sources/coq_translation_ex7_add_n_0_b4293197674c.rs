use vstd::prelude::*;

verus! {

pub proof fn ex7_add_n_0(n: nat)
    ensures add(n, 0nat) == n
    decreases n
{
    if n == 0 {
        assert(add(0nat, 0nat) == 0);
    } else {
        let n1 = (n - 1) as nat;
        ex7_add_n_0(n1);
        assert(add(n, 0nat) == add(n1, 0) + 1);
        assert(n == n1 + 1);
    }
}

} // verus!