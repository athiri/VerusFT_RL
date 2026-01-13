use vstd::prelude::*;

verus! {

pub open spec fn or_else<T>(o: Option<T>, alt: Option<T>) -> Option<T> {
    match o {
        Some(x) => Some(x),
        None => alt,
    }
}


pub proof fn or_else_none<T>(alt: Option<T>)
    ensures or_else::<T>(None, alt) == alt
{
}

} // verus!