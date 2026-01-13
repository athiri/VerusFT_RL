use vstd::prelude::*;

verus! {

pub open spec fn nandb(b1: bool, b2: bool) -> bool {
    negb(andb(b1, b2))
}

pub open spec fn andb(b1: bool, b2: bool) -> bool {
    b1 && b2
}

pub open spec fn negb(b: bool) -> bool {
    !b
}


pub proof fn ex4_nandb_def(b1: bool, b2: bool)
    ensures nandb(b1, b2) == !(b1 && b2)
{
    assert(nandb(b1, b2) == negb(andb(b1, b2)));
    assert(negb(andb(b1, b2)) == !(b1 && b2));
}

} // verus!