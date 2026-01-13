use vstd::prelude::*;

verus! {

pub open spec fn bool_to_dec(b: bool) -> Dec {
    if b { Dec::Yes } else { Dec::No }
}


pub enum Dec {
    Yes,
    No,
}

pub open spec fn dec_option_contains<T>(
    opt: Option<T>,
    x: T,
    eq: spec_fn(T, T) -> bool
) -> Dec {
    match opt {
        Option::None => Dec::No,
        Option::Some(v) => bool_to_dec(eq(v, x)),
    }
}

pub open spec fn dec_to_bool(d: Dec) -> bool {
    match d {
        Dec::Yes => true,
        Dec::No => false,
    }
}


pub proof fn dec_option_contains_sound<T>(opt: Option<T>, x: T, eq: spec_fn(T, T) -> bool)
    requires forall|a: T, b: T| #[trigger] eq(a, b) <==> (a == b)
    ensures dec_to_bool(dec_option_contains(opt, x, eq)) <==>
        (opt.is_some() && opt.unwrap() == x)
{
}

} // verus!