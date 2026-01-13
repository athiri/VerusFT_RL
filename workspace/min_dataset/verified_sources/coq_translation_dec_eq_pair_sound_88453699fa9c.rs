use vstd::prelude::*;

verus! {

pub open spec fn dec_to_bool(d: Dec) -> bool {
    match d {
        Dec::Yes => true,
        Dec::No => false,
    }
}

pub open spec fn dec_eq_pair<A, B>(
    p1: (A, B),
    p2: (A, B),
    dec_eq_a: spec_fn(A, A) -> Dec,
    dec_eq_b: spec_fn(B, B) -> Dec
) -> Dec {
    match (dec_eq_a(p1.0, p2.0), dec_eq_b(p1.1, p2.1)) {
        (Dec::Yes, Dec::Yes) => Dec::Yes,
        _ => Dec::No,
    }
}


pub enum Dec {
    Yes,
    No,
}


pub proof fn dec_eq_pair_sound<A, B>(
    p1: (A, B),
    p2: (A, B),
    dec_eq_a: spec_fn(A, A) -> Dec,
    dec_eq_b: spec_fn(B, B) -> Dec
)
    requires
        forall|x: A, y: A| #[trigger] dec_to_bool(dec_eq_a(x, y)) <==> (x == y),
        forall|x: B, y: B| #[trigger] dec_to_bool(dec_eq_b(x, y)) <==> (x == y),
    ensures
        dec_to_bool(dec_eq_pair(p1, p2, dec_eq_a, dec_eq_b)) <==> (p1 == p2)
{
}

} // verus!