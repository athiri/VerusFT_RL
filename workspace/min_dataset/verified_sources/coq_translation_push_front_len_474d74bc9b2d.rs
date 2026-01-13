use vstd::prelude::*;

verus! {

pub struct Deque { pub front: Seq<nat>, pub back: Seq<nat> }

pub open spec fn deque_len(d: Deque) -> nat { d.front.len() + d.back.len() }

pub open spec fn push_front(x: nat, d: Deque) -> Deque { Deque { front: seq![x] + d.front, back: d.back } }


pub proof fn push_front_len(x: nat, d: Deque) ensures deque_len(push_front(x, d)) == deque_len(d) + 1 {}

} // verus!