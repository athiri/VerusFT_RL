use vstd::prelude::*;

verus! {

pub open spec fn add(n: nat, m: nat) -> nat
    decreases n
{
    if n == 0 { m } else { add((n - 1) as nat, m) + 1 }
}

pub proof fn ex2_plus_n_Sm(n: nat, m: nat)
    ensures add(n, m + 1) == add(n, m) + 1
    decreases n
{
    if n > 0 {
        ex2_plus_n_Sm((n - 1) as nat, m);
    }
}

} // verus!
