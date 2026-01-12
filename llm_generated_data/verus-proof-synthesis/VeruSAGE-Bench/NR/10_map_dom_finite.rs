use vstd::prelude::*;

fn main() {}

verus! {

proof fn map_remove_dom<K, V>(m: Map<K, V>, k: K)
    ensures
        !m.remove(k).dom().contains(k),
{
}

}
