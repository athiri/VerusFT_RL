use vstd::prelude::*;

verus! {

pub open spec fn is_ok<T, E>(r: Result<T, E>) -> bool { matches!(r, Result::Ok(_)) }

pub open spec fn is_err<T, E>(r: Result<T, E>) -> bool { matches!(r, Result::Err(_)) }


pub proof fn err_not_ok<T, E>(e: E)
    ensures is_err(Result::<T, E>::Err(e)), !is_ok(Result::<T, E>::Err(e))
{}

} // verus!