use vstd::prelude::*;

fn main() {}

verus! {

proof fn map_contains_pair<K, V>(m: Map<K, V>, k: K, v: V)
    requires
        m.contains_key(k),
        m[k] == v,
    ensures
        m.contains_pair(k, v),
{
}

}
