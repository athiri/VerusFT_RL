use vstd::prelude::*;
fn main() {}
verus! {

// Map insert adds key to domain
pub proof fn map_insert_contains_key<K, V>(m: Map<K, V>, k: K, v: V)
    ensures
        m.insert(k, v).dom().contains(k),
{
}

} // verus!
