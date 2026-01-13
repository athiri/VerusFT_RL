use vstd::prelude::*;

fn main() {}

verus! {

proof fn map_remove_not_contains<K, V>(m: Map<K, V>, k: K)
    ensures
        !m.remove(k).contains_key(k),
{
}

}
