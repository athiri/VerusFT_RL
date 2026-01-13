use vstd::prelude::*;

verus! {

pub enum Trie<V> {
    Leaf,
    Node {
        value: Option<V>,
        left: Box<Trie<V>>,
        right: Box<Trie<V>>,
    },
}


pub open spec fn count_keys<V>(t: Trie<V>) -> nat
    decreases t
{
    match t {
        Trie::Leaf => 0,
        Trie::Node { value, left, right } => {
            let val_count = if value.is_some() { 1nat } else { 0nat };
            val_count + count_keys(*left) + count_keys(*right)
        }
    }
}

} // verus!