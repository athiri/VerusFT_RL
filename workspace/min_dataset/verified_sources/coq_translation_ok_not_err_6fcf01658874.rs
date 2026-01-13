use vstd::prelude::*;

verus! {

pub open spec fn is_ok<T, E>(r: Result<T, E>) -> bool { matches!(r, Result::Ok(_)) }

pub open spec fn is_err<T, E>(r: Result<T, E>) -> bool { matches!(r, Result::Err(_)) }


pub proof fn ok_not_err<T, E>(t: T)
    ensures is_ok(Result::<T, E>::Ok(t)), !is_err(Result::<T, E>::Ok(t))
{}

} // verus!