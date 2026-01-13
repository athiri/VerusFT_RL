use vstd::prelude::*;

verus! {

pub open spec fn andb(b1: bool, b2: bool) -> bool {
    b1 && b2
}


pub proof fn ex2_andb_comm(b1: bool, b2: bool)
    ensures andb(b1, b2) == andb(b2, b1)
{
    match (b1, b2) {
        (true, true) => {
            assert(andb(b1, b2));
            assert(andb(b2, b1));
        }
        (true, false) => {
            assert(!andb(b1, b2));
            assert(!andb(b2, b1));
        }
        (false, true) => {
            assert(!andb(b1, b2));
            assert(!andb(b2, b1));
        }
        (false, false) => {
            assert(!andb(b1, b2));
            assert(!andb(b2, b1));
        }
    }
}

} // verus!