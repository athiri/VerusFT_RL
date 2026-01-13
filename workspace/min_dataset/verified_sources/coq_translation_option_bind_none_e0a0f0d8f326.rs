use vstd::prelude::*;

verus! {

pub open spec fn option_bind<T, U>(opt: Option<T>, f: spec_fn(T) -> Option<U>) -> Option<U> {
    match opt {
        Option::None => Option::None,
        Option::Some(v) => f(v),
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

pub open spec fn dec_is_none<T>(opt: Option<T>) -> Dec {
    match opt {
        Option::Some(_) => Dec::No,
        Option::None => Dec::Yes,
    }
}


pub proof fn option_bind_none<T, U>(f: spec_fn(T) -> Option<U>)
    ensures dec_to_bool(dec_is_none(option_bind(Option::<T>::None, f)))
{
}

} // verus!