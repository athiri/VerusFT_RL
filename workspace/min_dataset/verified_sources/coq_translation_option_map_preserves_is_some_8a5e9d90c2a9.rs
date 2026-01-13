use vstd::prelude::*;

verus! {

pub open spec fn option_map<T, U>(opt: Option<T>, f: spec_fn(T) -> U) -> Option<U> {
    match opt {
        Option::None => Option::None,
        Option::Some(v) => Option::Some(f(v)),
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

pub open spec fn dec_is_some<T>(opt: Option<T>) -> Dec {
    match opt {
        Option::Some(_) => Dec::Yes,
        Option::None => Dec::No,
    }
}


pub proof fn option_map_preserves_is_some<T, U>(opt: Option<T>, f: spec_fn(T) -> U)
    ensures dec_to_bool(dec_is_some(opt)) == dec_to_bool(dec_is_some(option_map(opt, f)))
{
}

} // verus!