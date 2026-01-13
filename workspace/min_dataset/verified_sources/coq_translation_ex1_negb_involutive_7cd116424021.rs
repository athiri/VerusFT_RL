use vstd::prelude::*;

verus! {

pub open spec fn negb(b: bool) -> bool { !b }


pub proof fn ex1_negb_involutive(b: bool)
    ensures negb(negb(b)) == b
{
    match b {
        true => {
            assert(negb(true) == false);
            assert(negb(negb(true)) == true);
        }
        false => {
            assert(negb(false) == true);
            assert(negb(negb(false)) == false);
        }
    }
}

} // verus!