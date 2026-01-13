use vstd::prelude::*;

verus! {

pub struct Deque { pub front: Seq<nat>, pub back: Seq<nat> }

pub open spec fn deque_len(d: Deque) -> nat { d.front.len() + d.back.len() }

pub open spec fn push_back(d: Deque, x: nat) -> Deque { Deque { front: d.front, back: d.back.push(x) } }


pub proof fn push_back_len(d: Deque, x: nat) ensures deque_len(push_back(d, x)) == deque_len(d) + 1 {}

} // verus!