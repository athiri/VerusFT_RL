use vstd::prelude::*;

verus! {

pub open spec fn negb(b: bool) -> bool {
    !b
}


pub proof fn ex1_negb_involutive(b: bool)
    ensures negb(negb(b)) == b
{
    if b {
        assert(negb(b) == false);
        assert(negb(negb(b)) == true);
    } else {
        assert(negb(b) == true);
        assert(negb(negb(b)) == false);
    }
}

} // verus!