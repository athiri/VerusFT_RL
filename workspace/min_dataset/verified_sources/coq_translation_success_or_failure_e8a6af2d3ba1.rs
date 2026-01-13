use vstd::prelude::*;

verus! {

pub enum MyResult<A> {
    Success { value: A },
    Failure,
}


pub open spec fn is_failure<A>(r: MyResult<A>) -> bool {
    match r { MyResult::Failure { .. } => true, _ => false }
}

pub open spec fn is_success<A>(r: MyResult<A>) -> bool {
    match r { MyResult::Success { .. } => true, _ => false }
}


pub proof fn success_or_failure<A>(r: MyResult<A>)
    ensures is_success(r) || is_failure(r)
{
    match r {
        MyResult::Success { .. } => {}
        MyResult::Failure { .. } => {}
    }
}

} // verus!