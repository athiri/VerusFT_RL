use vstd::prelude::*;

verus! {

pub open spec fn bucket_contains(bucket: Seq<(nat, nat)>, k: nat) -> bool decreases bucket.len() {
    bucket.len() > 0 && (bucket[0].0 == k || bucket_contains(bucket.skip(1), k))
}

} // verus!