use vstd::prelude::*;

fn main() {}

verus! {

proof fn map_insert_lookup<K, V>(m: Map<K, V>, k: K, v: V)
    ensures
        m.insert(k, v)[k] == v,
{
}

}
