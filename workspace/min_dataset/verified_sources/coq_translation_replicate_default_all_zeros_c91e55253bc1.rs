use vstd::prelude::*;

verus! {

pub open spec fn default_nat() -> nat {
    0
}


pub open spec fn replicate_default_nat(n: nat) -> Seq<nat>
    decreases n
{
    if n == 0 {
        Seq::empty()
    } else {
        seq![default_nat()].add(replicate_default_nat((n - 1) as nat))
    }
}

pub proof fn replicate_default_length(n: nat)
    ensures replicate_default_nat(n).len() == n
    decreases n
{
    if n == 0 {
        assert(replicate_default_nat(0) =~= Seq::empty());
    } else {
        replicate_default_length((n - 1) as nat);
        assert(replicate_default_nat(n).len() == 1 + replicate_default_nat((n - 1) as nat).len());
    }
}


pub proof fn replicate_default_all_zeros(n: nat, i: nat)
    requires i < n
    ensures replicate_default_nat(n)[i as int] == 0
    decreases n
{
    replicate_default_length(n);
    if n == 0 {
        // vacuously true
    } else if i == 0 {
        assert(replicate_default_nat(n)[0] == 0);
    } else {
        replicate_default_length((n - 1) as nat);
        replicate_default_all_zeros((n - 1) as nat, (i - 1) as nat);
        // Show the structure of replicate_default_nat(n)
        let rest = replicate_default_nat((n - 1) as nat);
        assert(replicate_default_nat(n) == seq![default_nat()].add(rest));
        assert(rest.len() == (n - 1) as nat);
        assert(i - 1 < rest.len());
        assert(seq![default_nat()].add(rest)[i as int] == rest[(i - 1) as int]);
    }
}

} // verus!