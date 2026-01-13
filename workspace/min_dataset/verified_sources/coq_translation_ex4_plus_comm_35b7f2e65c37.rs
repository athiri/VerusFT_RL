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

pub proof fn ex1_plus_n_O(n: nat)
    ensures add(n, 0nat) == n
    decreases n
{
    if n == 0 {
        assert(add(0nat, 0nat) == 0);
    } else {
        let n1 = (n - 1) as nat;
        ex1_plus_n_O(n1);
        assert(add(n, 0nat) == add(n1, 0) + 1);
        assert(n == n1 + 1);
    }
}


pub proof fn ex4_plus_comm(a: nat, b: nat)
    ensures add(a, b) == add(b, a)
    decreases a
{
    if a == 0 {
        assert(add(0, b) == b);
        ex1_plus_n_O(b);
        assert(add(b, 0) == b);
    } else {
        let a1 = (a - 1) as nat;
        ex4_plus_comm(a1, b);

        // LHS: add(a1+1,b) = add(a1,b)+1
        assert(add(a, b) == add(a1, b) + 1);

        // RHS: add(b,a1+1) = add(b,a1)+1 (succ on the right)
        ex2_plus_n_Sm(b, a1);
        assert(add(b, a) == add(b, a1) + 1);

        // Use IH: add(a1,b) = add(b,a1)
        assert(add(a1, b) == add(b, a1));
    }
}

} // verus!