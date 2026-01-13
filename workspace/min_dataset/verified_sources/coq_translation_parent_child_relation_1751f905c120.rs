use vstd::prelude::*;

verus! {

pub open spec fn right_child(i: nat) -> nat { 2 * i + 2 }


pub open spec fn parent(i: nat) -> nat { if i == 0 { 0 } else { ((i - 1) / 2) as nat } }

pub open spec fn left_child(i: nat) -> nat { 2 * i + 1 }


pub proof fn parent_child_relation(i: nat)
    requires i > 0
    ensures parent(left_child(parent(i))) == parent(i) || parent(right_child(parent(i))) == parent(i)
{}

} // verus!