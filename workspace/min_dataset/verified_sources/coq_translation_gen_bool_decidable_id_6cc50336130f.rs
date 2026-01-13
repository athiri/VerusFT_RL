use vstd::prelude::*;

verus! {

pub open spec fn gen_bool_outputs() -> Set<bool> {
    set![true, false]
}

pub open spec fn decidable_id(b: bool) -> bool {
    b
}


pub proof fn gen_bool_decidable_id()
    ensures forall|b: bool| gen_bool_outputs().contains(b) ==>
        (decidable_id(b) == true <==> b == true)
{
}

} // verus!