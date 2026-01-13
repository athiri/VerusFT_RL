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


pub open spec fn lookup<V>(t: Trie<V>, key: Seq<bool>) -> Option<V>
    decreases key.len()
{
    match t {
        Trie::Leaf => None,
        Trie::Node { value, left, right } =>
            if key.len() == 0 {
                value
            } else if key[0] {
                lookup(*right, key.skip(1))
            } else {
                lookup(*left, key.skip(1))
            }
    }
}

} // verus!