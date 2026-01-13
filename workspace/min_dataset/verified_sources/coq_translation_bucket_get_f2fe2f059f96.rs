use vstd::prelude::*;

verus! {

pub open spec fn bucket_get(bucket: Seq<(nat, nat)>, k: nat) -> Option<nat> decreases bucket.len() {
    if bucket.len() == 0 { None }
    else if bucket[0].0 == k { Some(bucket[0].1) }
    else { bucket_get(bucket.skip(1), k) }
}

} // verus!