use vstd::prelude::*;

verus! {

pub type Id = nat;

pub open spec fn id_lt(x: Id, y: Id) -> bool {
    x < y
}


pub proof fn id_lt_trans(x: Id, y: Id, z: Id)
    requires
        id_lt(x, y),
        id_lt(y, z),
    ensures id_lt(x, z)
{
}

} // verus!