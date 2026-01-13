use vstd::prelude::*;

verus! {

pub open spec fn dec_or(d1: Dec, d2: Dec) -> Dec {
    match (d1, d2) {
        (Dec::No, Dec::No) => Dec::No,
        _ => Dec::Yes,
    }
}


pub enum Dec {
    Yes,
    No,
}

pub open spec fn dec_or_lazy(d1: Dec, d2_thunk: spec_fn() -> Dec) -> Dec {
    match d1 {
        Dec::Yes => Dec::Yes,
        Dec::No => d2_thunk(),
    }
}


pub proof fn dec_or_lazy_equiv(d1: Dec, d2: Dec)
    ensures dec_or_lazy(d1, || d2) == dec_or(d1, d2)
{
}

} // verus!