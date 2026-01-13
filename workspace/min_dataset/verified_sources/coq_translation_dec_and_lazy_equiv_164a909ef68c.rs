use vstd::prelude::*;

verus! {

pub open spec fn dec_and(d1: Dec, d2: Dec) -> Dec {
    match (d1, d2) {
        (Dec::Yes, Dec::Yes) => Dec::Yes,
        _ => Dec::No,
    }
}


pub enum Dec {
    Yes,
    No,
}

pub open spec fn dec_and_lazy(d1: Dec, d2_thunk: spec_fn() -> Dec) -> Dec {
    match d1 {
        Dec::No => Dec::No,
        Dec::Yes => d2_thunk(),
    }
}


pub proof fn dec_and_lazy_equiv(d1: Dec, d2: Dec)
    ensures dec_and_lazy(d1, || d2) == dec_and(d1, d2)
{
}

} // verus!