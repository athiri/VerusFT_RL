use vstd::prelude::*;

verus! {

pub struct KeyValue { pub key: nat, pub value: nat }


pub open spec fn filter_key(s: Seq<KeyValue>, k: nat) -> Seq<KeyValue> decreases s.len() {
    if s.len() == 0 { Seq::empty() }
    else if s[0].key == k { seq![s[0]] + filter_key(s.skip(1), k) }
    else { filter_key(s.skip(1), k) }
}

} // verus!