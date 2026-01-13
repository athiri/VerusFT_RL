use vstd::prelude::*;

verus! {

pub open spec fn shrink_nat(n: nat) -> Seq<nat>
    decreases n
{
    if n == 0 {
        Seq::empty()
    } else {
        let half = (n / 2) as nat;
        if half == 0 {
            seq![0nat]
        } else {
            seq![0nat, half]
        }
    }
}

pub open spec fn shrink_pair_nat(p: (nat, nat)) -> Seq<(nat, nat)> {
    let (a, b) = p;
    let shrink_a = shrink_nat(a);
    let shrink_b = shrink_nat(b);

    // Shrink first component
    let first_shrinks = Seq::new(shrink_a.len(), |i: int| (shrink_a[i], b));
    // Shrink second component
    let second_shrinks = Seq::new(shrink_b.len(), |i: int| (a, shrink_b[i]));

    first_shrinks.add(second_shrinks)
}


pub proof fn shrink_pair_nonzero_has_candidates(a: nat, b: nat)
    requires a > 0 || b > 0
    ensures shrink_pair_nat((a, b)).len() > 0
{
    let shrink_a = shrink_nat(a);
    let shrink_b = shrink_nat(b);

    if a > 0 {
        // shrink_nat(a) has at least 1 element when a > 0
        if a == 1 {
            assert(shrink_nat(1) =~= seq![0nat]);
            assert(shrink_a.len() == 1);
        } else {
            assert(shrink_a.len() >= 1);
        }
    }
    if b > 0 {
        if b == 1 {
            assert(shrink_nat(1) =~= seq![0nat]);
            assert(shrink_b.len() == 1);
        } else {
            assert(shrink_b.len() >= 1);
        }
    }
}

} // verus!