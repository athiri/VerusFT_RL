use vstd::prelude::*;

verus! {

pub proof fn lemma_add_assoc(a: nat, b: nat, c: nat)
    ensures add(add(a, b), c) == add(a, add(b, c))
    decreases a
{
    if a == 0 {
    } else {
        lemma_add_assoc((a - 1) as nat, b, c);
    }
}

} // verus!