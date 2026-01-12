use vstd::prelude::*;

fn main() {}

verus! {

proof fn map_insert_contains<K, V>(m: Map<K, V>, k: K, v: V)
    ensures
        m.insert(k, v).contains_key(k),
{
}

}
