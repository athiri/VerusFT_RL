use vstd::prelude::*;

verus! {

pub proof fn lemma_add_succ_l(x: nat, m: nat)
    ensures add(x + 1, m) == add(x, m) + 1
{
    // `x + 1` is always nonzero for nat, so unfolding is safe.
    assert(add(x + 1, m) == add(x, m) + 1);
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

} // verus!