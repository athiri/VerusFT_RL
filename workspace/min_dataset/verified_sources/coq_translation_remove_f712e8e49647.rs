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


pub open spec fn remove<V>(t: Trie<V>, key: Seq<bool>) -> Trie<V>
    decreases key.len()
{
    match t {
        Trie::Leaf => Trie::Leaf,
        Trie::Node { value, left, right } =>
            if key.len() == 0 {
                Trie::Node {
                    value: None,
                    left,
                    right,
                }
            } else if key[0] {
                Trie::Node {
                    value,
                    left,
                    right: Box::new(remove(*right, key.skip(1))),
                }
            } else {
                Trie::Node {
                    value,
                    left: Box::new(remove(*left, key.skip(1))),
                    right,
                }
            }
    }
}

} // verus!