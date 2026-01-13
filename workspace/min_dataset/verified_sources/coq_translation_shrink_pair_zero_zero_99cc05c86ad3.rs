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


pub proof fn shrink_pair_zero_zero()
    ensures shrink_pair_nat((0, 0)).len() == 0
{
    assert(shrink_nat(0).len() == 0);
    let result = shrink_pair_nat((0, 0));
    let first_shrinks = Seq::new(0nat, |i: int| (0nat, 0nat));
    let second_shrinks = Seq::new(0nat, |i: int| (0nat, 0nat));
    assert(first_shrinks =~= Seq::<(nat, nat)>::empty());
    assert(second_shrinks =~= Seq::<(nat, nat)>::empty());
}

} // verus!