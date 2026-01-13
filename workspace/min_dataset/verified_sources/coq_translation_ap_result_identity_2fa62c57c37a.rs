use vstd::prelude::*;

verus! {

pub open spec fn ap_result<A, B, E>(
    rf: Result<spec_fn(A) -> B, E>,
    ra: Result<A, E>
) -> Result<B, E> {
    match rf {
        Result::Err(e) => Result::Err(e),
        Result::Ok(f) => match ra {
            Result::Err(e) => Result::Err(e),
            Result::Ok(a) => Result::Ok(f(a)),
        }
    }
}


pub proof fn ap_result_identity<A, E>(v: Result<A, E>)
    ensures ap_result::<A, A, E>(Result::Ok(|a: A| a), v) == v
{
    match v {
        Result::Err(e) => {}
        Result::Ok(x) => {}
    }
}

} // verus!