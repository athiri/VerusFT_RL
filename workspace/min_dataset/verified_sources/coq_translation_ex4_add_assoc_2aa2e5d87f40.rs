use vstd::prelude::*;

verus! {

pub proof fn ex4_add_assoc(a: nat, b: nat, c: nat)
    ensures add(add(a, b), c) == add(a, add(b, c))
    decreases a
{
    if a == 0 {
        assert(add(add(0, b), c) == add(b, c));
        assert(add(0, add(b, c)) == add(b, c));
    } else {
        let a1 = (a - 1) as nat;
        ex4_add_assoc(a1, b, c);

        assert(add(a, b) == add(a1, b) + 1);
        assert(add(add(a, b), c) == add(add(a1, b) + 1, c));
        assert(add(a, add(b, c)) == add(a1, add(b, c)) + 1);

        // A tiny rewrite: add(x+1, c) unfolds to add(x,c)+1
        assert(add(add(a1, b) + 1, c) == add(add(a1, b), c) + 1);

        // Close with IH
        assert(add(add(a1, b), c) == add(a1, add(b, c)));
    }
}

} // verus!