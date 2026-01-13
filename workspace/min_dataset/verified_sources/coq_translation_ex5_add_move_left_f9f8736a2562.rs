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

pub proof fn lemma_add_succ_l(x: nat, m: nat)
    ensures add(x + 1, m) == add(x, m) + 1
{
    // `x + 1` is always nonzero for nat, so unfolding is safe.
    assert(add(x + 1, m) == add(x, m) + 1);
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


pub proof fn ex3_plus_assoc(a: nat, b: nat, c: nat)
    ensures add(add(a, b), c) == add(a, add(b, c))
    decreases a
{
    if a == 0 {
        assert(add(add(0, b), c) == add(b, c));
        assert(add(0, add(b, c)) == add(b, c));
    } else {
        let a1 = (a - 1) as nat;
        ex3_plus_assoc(a1, b, c);

        assert(a == a1 + 1);
        assert(add(a, b) == add(a1, b) + 1);

        // Rewrite LHS using succ-left lemma on (add(a1,b)).
        lemma_add_succ_l(add(a1, b), c);
        assert(add(add(a, b), c) == add(add(a1, b), c) + 1);

        // Rewrite RHS using definitional unfolding at `a = a1 + 1`.
        assert(add(a, add(b, c)) == add(a1, add(b, c)) + 1);

        // Close via IH.
        assert(add(add(a1, b), c) == add(a1, add(b, c)));
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


pub proof fn ex5_add_move_left(x: nat, y: nat, z: nat)
    ensures add(x, add(y, z)) == add(y, add(x, z))
{
    // add(x, add(y,z)) = add(add(x,y), z)
    ex3_plus_assoc(x, y, z);
    assert(add(x, add(y, z)) == add(add(x, y), z));

    // swap x and y in add(x,y)
    ex4_plus_comm(x, y);
    assert(add(x, y) == add(y, x));

    // add(add(y,x), z) = add(y, add(x,z))
    ex3_plus_assoc(y, x, z);
    assert(add(add(y, x), z) == add(y, add(x, z)));

    // Now the solver can connect the equalities.
}

} // verus!