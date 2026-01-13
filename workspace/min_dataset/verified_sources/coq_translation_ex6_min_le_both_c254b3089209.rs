use vstd::prelude::*;

verus! {

pub open spec fn min(x: int, y: int) -> int { if x <= y { x } else { y } }


pub proof fn ex6_min_le_both()
    ensures forall|i: int, j: int| min(i, j) <= i && min(i, j) <= j
{
    assert forall|i: int, j: int| min(i, j) <= i && min(i, j) <= j by {
        if i <= j {
            assert(min(i, j) == i);
        } else {
            assert(min(i, j) == j);
        }
    };
}

} // verus!