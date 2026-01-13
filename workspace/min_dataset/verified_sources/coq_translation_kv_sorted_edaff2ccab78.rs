use vstd::prelude::*;

verus! {

pub struct KeyValue { pub key: nat, pub value: nat }

pub open spec fn kv_le(a: KeyValue, b: KeyValue) -> bool { a.key <= b.key }


pub open spec fn kv_sorted(s: Seq<KeyValue>) -> bool decreases s.len() {
    s.len() <= 1 || (kv_le(s[0], s[1]) && kv_sorted(s.skip(1)))
}

} // verus!