use vstd::prelude::*;

verus! {

pub open spec fn option_or<T>(a: Option<T>, b: Option<T>) -> Option<T> {
    match a {
        Option::Some(_) => a,
        Option::None => b,
    }
}


pub enum Dec {
    Yes,
    No,
}

pub open spec fn dec_option_or_is_some<T>(a: Option<T>, b: Option<T>) -> Dec {
    dec_is_some(option_or(a, b))
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


pub proof fn option_or_some<T>(a: Option<T>, b: Option<T>)
    ensures dec_to_bool(dec_option_or_is_some(a, b)) <==>
        (dec_to_bool(dec_is_some(a)) || dec_to_bool(dec_is_some(b)))
{
}

} // verus!