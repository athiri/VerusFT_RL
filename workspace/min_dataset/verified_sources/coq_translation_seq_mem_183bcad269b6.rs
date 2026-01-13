use vstd::prelude::*;

verus! {

pub open spec fn seq_mem(s: Seq<nat>, x: nat) -> bool decreases s.len() {
    s.len() > 0 && (s[0] == x || seq_mem(s.skip(1), x))
}

} // verus!