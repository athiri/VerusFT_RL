use vstd::prelude::*;

verus! {

pub open spec fn dec_eq_option<T>(
    a: Option<T>,
    b: Option<T>,
    dec_eq_t: spec_fn(T, T) -> Dec
) -> Dec {
    match (a, b) {
        (Option::None, Option::None) => Dec::Yes,
        (Option::Some(x), Option::Some(y)) => dec_eq_t(x, y),
        _ => Dec::No,
    }
}


pub enum Dec {
    Yes,
    No,
}

pub open spec fn dec_to_bool(d: Dec) -> bool {
    match d {
        Dec::Yes => true,
        Dec::No => false,
    }
}


pub proof fn dec_eq_option_sound<T>(
    a: Option<T>,
    b: Option<T>,
    dec_eq_t: spec_fn(T, T) -> Dec,
    eq_t: spec_fn(T, T) -> bool
)
    requires
        forall|x: T, y: T| #[trigger] dec_to_bool(dec_eq_t(x, y)) <==> eq_t(x, y),
        forall|x: T, y: T| #[trigger] eq_t(x, y) <==> (x == y),
    ensures
        dec_to_bool(dec_eq_option(a, b, dec_eq_t)) <==> (a == b)
{
}

} // verus!