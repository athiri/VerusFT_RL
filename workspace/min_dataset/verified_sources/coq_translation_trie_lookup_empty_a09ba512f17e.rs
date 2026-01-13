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

pub open spec fn trie_lookup<V>(t: Trie<V>, key: Seq<bool>) -> Option<V>
    decreases key.len()
{
    match t {
        Trie::Leaf => None,
        Trie::Node { value, left, right } =>
            if key.len() == 0 {
                value
            } else if key[0] {
                trie_lookup(*right, key.skip(1))
            } else {
                trie_lookup(*left, key.skip(1))
            }
    }
}

pub open spec fn trie_empty<V>() -> Trie<V> {
    Trie::Leaf
}


pub proof fn trie_lookup_empty<V>(key: Seq<bool>)
    ensures trie_lookup::<V>(trie_empty(), key).is_none()
{
}

} // verus!