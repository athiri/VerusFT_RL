use vstd::prelude::*;

verus! {

pub proof fn ex3_add_n_0(n: nat)
    ensures add(n, 0nat) == n
    decreases n
{
    if n == 0 {
        assert(add(0nat, 0nat) == 0);
    } else {
        let n1 = (n - 1) as nat;
        ex3_add_n_0(n1);
        assert(add(n, 0nat) == add(n1, 0) + 1);
        assert(n == n1 + 1);
    }
}


pub proof fn ex9_exists_nat()
    ensures exists|k: nat| #[trigger] add(k, 0) == k
{
    assert(exists|k: nat| #[trigger] add(k, 0) == k) by {
        let w: nat = 0;
        ex3_add_n_0(w);
        assert(add(w, 0) == w);
    };
}

} // verus!