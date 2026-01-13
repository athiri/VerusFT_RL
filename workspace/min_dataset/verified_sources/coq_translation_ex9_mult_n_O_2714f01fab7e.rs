use vstd::prelude::*;

verus! {

pub proof fn ex1_plus_n_O(n: nat)
    ensures add(n, 0nat) == n
    decreases n
{
    if n == 0 {
        assert(add(0nat, 0nat) == 0);
    } else {
        let n1 = (n - 1) as nat;
        ex1_plus_n_O(n1);
        assert(add(n, 0nat) == add(n1, 0nat) + 1);
        assert(n == n1 + 1);
    }
}


pub proof fn ex9_mult_n_O(n: nat)
    ensures mul(n, 0nat) == 0
    decreases n
{
    if n == 0 {
        assert(mul(0nat, 0nat) == 0);
    } else {
        let n1 = (n - 1) as nat;
        ex9_mult_n_O(n1);
        assert(mul(n, 0nat) == add(mul(n1, 0nat), 0nat));
        ex1_plus_n_O(mul(n1, 0nat));
    }
}

} // verus!