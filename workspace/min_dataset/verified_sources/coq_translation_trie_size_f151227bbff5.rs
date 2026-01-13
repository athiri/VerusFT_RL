use vstd::prelude::*;

verus! {

pub enum Trie<V> {
    Leaf,
    Node {
        value: Option<V>,
        left: Box<Trie<V>>,   // false branch
        right: Box<Trie<V>>,  // true branch
    },
}


pub open spec fn trie_size<V>(t: Trie<V>) -> nat
    decreases t
{
    match t {
        Trie::Leaf => 0,
        Trie::Node { value, left, right } => {
            let val_count = if value.is_some() { 1nat } else { 0nat };
            val_count + trie_size(*left) + trie_size(*right)
        }
    }
}

} // verus!