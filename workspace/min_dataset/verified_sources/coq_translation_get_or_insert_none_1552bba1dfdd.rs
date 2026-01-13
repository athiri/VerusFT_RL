use vstd::prelude::*;

verus! {

pub open spec fn get_or_insert<A>(o: Option<A>, default: A) -> (Option<A>, A) {
    match o {
        Option::None => (Option::Some(default), default),
        Option::Some(x) => (Option::Some(x), x),
    }
}


pub proof fn get_or_insert_none(default: nat)
    ensures get_or_insert(Option::<nat>::None, default) == (Option::Some(default), default)
{
    // Trivially true
}

} // verus!