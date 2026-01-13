use vstd::prelude::*;

verus! {

pub struct RingBuffer { pub data: Seq<nat>, pub head: nat, pub len: nat, pub cap: nat }

pub open spec fn rb_is_empty(rb: RingBuffer) -> bool { rb.len == 0 }

pub open spec fn rb_empty(cap: nat) -> RingBuffer recommends cap > 0 {
    RingBuffer { data: Seq::new(cap, |_i: int| 0nat), head: 0, len: 0, cap }
}


pub proof fn rb_empty_is_empty(cap: nat) requires cap > 0 ensures rb_is_empty(rb_empty(cap)) {}

} // verus!