use vstd::prelude::*;

verus! {

pub open spec fn arbitrary_bool() -> Set<bool> {
    Set::new(|b: bool| b == true || b == false)
}


pub proof fn arbitrary_bool_complete()
    ensures
        arbitrary_bool().contains(true),
        arbitrary_bool().contains(false)
{
}

} // verus!