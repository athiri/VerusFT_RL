use vstd::prelude::*;

verus! {

pub open spec fn hash(k: nat, num_buckets: nat) -> nat recommends num_buckets > 0 { k % num_buckets }


pub proof fn hash_bound(k: nat, n: nat) requires n > 0 ensures hash(k, n) < n {}

} // verus!