use vstd::prelude::*;

verus! {

pub open spec fn pure_result<A, E>(a: A) -> Result<A, E> {
    Result::Ok(a)
}

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


pub proof fn ap_result_homomorphism<A, B, E>(f: spec_fn(A) -> B, x: A)
    ensures ap_result::<A, B, E>(pure_result(f), pure_result(x)) == pure_result::<B, E>(f(x))
{
    // Trivially true
}

} // verus!