use vstd::prelude::*;

verus! {

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

pub open spec fn nat_measure(n: nat) -> nat {
    n
}


pub proof fn shrink_nat_decreases_measure(n: nat, i: int)
    requires 0 <= i < shrink_nat(n).len() as int
    ensures nat_measure(shrink_nat(n)[i]) < nat_measure(n)
{
    shrink_nat_smaller(n, i);
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

pub proof fn shrink_seq_singleton(n: nat)
    ensures shrink_seq_nat(seq![n]).len() == 1,
            shrink_seq_nat(seq![n])[0] =~= Seq::empty()
{
    let xs = seq![n];
    assert(xs.len() == 1);
    assert(shrink_seq_nat(xs) =~= seq![Seq::<nat>::empty()]);
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

pub proof fn shrink_seq_removes_elements(xs: Seq<nat>, i: int)
    requires xs.len() > 1,
             0 <= i < xs.len() as int
    ensures shrink_seq_nat(xs)[i].len() == xs.len() - 1
{
    let removes = Seq::new(xs.len(), |j: int| xs.remove(j));
    assert(shrink_seq_nat(xs) =~= removes);
    assert(removes[i] == xs.remove(i));
    assert(xs.remove(i).len() == xs.len() - 1);
}


pub open spec fn shrink_seq_nat(xs: Seq<nat>) -> Seq<Seq<nat>>
    decreases xs.len()
{
    if xs.len() == 0 {
        Seq::empty()
    } else if xs.len() == 1 {
        // Single element: try empty sequence
        seq![Seq::empty()]
    } else {
        // Multiple elements: try removing each element
        let removes = Seq::new(xs.len(), |i: int| xs.remove(i));
        removes
    }
}

pub open spec fn shrink_bool(b: bool) -> Seq<bool> {
    if b {
        seq![false]  // true shrinks to false
    } else {
        Seq::empty()  // false doesn't shrink
    }
}


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

pub open spec fn shrink_option_nat(o: Option<nat>) -> Seq<Option<nat>> {
    match o {
        Option::None => Seq::empty(),
        Option::Some(n) => {
            // Shrink to None first, then shrink the inner value
            let shrunk_inner = shrink_nat(n);
            // Add Some(x) for each shrunk inner value
            Seq::new(shrunk_inner.len() + 1, |i: int|
                if i == 0 {
                    Option::<nat>::None
                } else {
                    Option::Some(shrunk_inner[i - 1])
                }
            )
        }
    }
}


pub proof fn shrink_nat_zero()
    ensures shrink_nat(0).len() == 0
{
    assert(shrink_nat(0) =~= Seq::empty());
}

pub proof fn shrink_option_some_includes_none(n: nat)
    ensures shrink_option_nat(Option::Some(n)).len() > 0,
            shrink_option_nat(Option::Some(n))[0] == Option::<nat>::None
{
    let result = shrink_option_nat(Option::Some(n));
    assert(result.len() == shrink_nat(n).len() + 1);
    assert(result[0] == Option::<nat>::None);
}

pub proof fn shrink_nat_smaller(n: nat, i: int)
    requires 0 <= i < shrink_nat(n).len() as int
    ensures shrink_nat(n)[i] < n
{
    if n == 0 {
        // Vacuously true - empty sequence
    } else {
        let half = (n / 2) as nat;
        if half == 0 {
            assert(shrink_nat(n) =~= seq![0nat]);
            assert(shrink_nat(n)[0] == 0);
            assert(0 < n);
        } else {
            assert(shrink_nat(n) =~= seq![0nat, half]);
            if i == 0 {
                assert(shrink_nat(n)[0] == 0);
                assert(0 < n);
            } else {
                assert(shrink_nat(n)[1] == half);
                assert(half < n);
            }
        }
    }
}

pub proof fn shrink_bool_law()
    ensures shrink_bool(false).len() == 0,
            shrink_bool(true).len() == 1,
            shrink_bool(true)[0] == false
{
    assert(shrink_bool(false) =~= Seq::empty());
    assert(shrink_bool(true) =~= seq![false]);
}

pub proof fn shrink_option_none()
    ensures shrink_option_nat(Option::None).len() == 0
{
    // Option::None doesn't shrink
}

pub proof fn shrink_seq_empty()
    ensures shrink_seq_nat(Seq::empty()).len() == 0
{
    assert(shrink_seq_nat(Seq::empty()) =~= Seq::empty());
}


pub proof fn example_shrink()
{
    // Nat shrinking
    shrink_nat_zero();
    shrink_nat_smaller(10, 0);
    shrink_nat_smaller(10, 1);

    // Bool shrinking
    shrink_bool_law();

    // Option shrinking
    shrink_option_none();
    shrink_option_some_includes_none(5);

    // Seq shrinking
    shrink_seq_empty();
    shrink_seq_singleton(42);
    shrink_seq_removes_elements(seq![1nat, 2, 3], 1);

    // Pair shrinking
    shrink_pair_zero_zero();
    shrink_pair_nonzero_has_candidates(5, 0);

    // Measure decreases
    shrink_nat_decreases_measure(100, 0);
}

} // verus!