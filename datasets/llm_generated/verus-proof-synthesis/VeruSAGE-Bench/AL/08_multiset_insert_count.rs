use vstd::{multiset::*, prelude::*};

fn main() {}

verus! {

proof fn multiset_insert_count<V>(m: Multiset<V>, v: V)
    ensures
        m.insert(v).count(v) == m.count(v) + 1,
{
}

}
