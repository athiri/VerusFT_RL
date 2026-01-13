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
        assert(add(n, 0nat) == add(n1, 0nat) + 1);
        assert(n == n1 + 1);
    }
}


pub proof fn ex10_mul_n_0(n: nat)
    ensures mul(n, 0nat) == 0
    decreases n
{
    if n == 0 {
        assert(mul(0nat, 0nat) == 0);
    } else {
        let n1 = (n - 1) as nat;
        ex10_mul_n_0(n1);
        assert(mul(n, 0nat) == add(mul(n1, 0nat), 0nat));
        // add(x, 0nat) == x (proved above)
        ex7_add_n_0(mul(n1, 0nat));
    }
}

} // verus!