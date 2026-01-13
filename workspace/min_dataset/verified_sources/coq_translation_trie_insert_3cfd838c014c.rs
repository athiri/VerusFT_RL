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


pub open spec fn trie_insert<V>(t: Trie<V>, key: Seq<bool>, v: V) -> Trie<V>
    decreases key.len()
{
    if key.len() == 0 {
        match t {
            Trie::Leaf => Trie::Node {
                value: Some(v),
                left: Box::new(Trie::Leaf),
                right: Box::new(Trie::Leaf),
            },
            Trie::Node { value: _, left, right } => Trie::Node {
                value: Some(v),
                left,
                right,
            },
        }
    } else {
        match t {
            Trie::Leaf => {
                if key[0] {
                    Trie::Node {
                        value: None,
                        left: Box::new(Trie::Leaf),
                        right: Box::new(trie_insert(Trie::Leaf, key.skip(1), v)),
                    }
                } else {
                    Trie::Node {
                        value: None,
                        left: Box::new(trie_insert(Trie::Leaf, key.skip(1), v)),
                        right: Box::new(Trie::Leaf),
                    }
                }
            }
            Trie::Node { value, left, right } => {
                if key[0] {
                    Trie::Node {
                        value,
                        left,
                        right: Box::new(trie_insert(*right, key.skip(1), v)),
                    }
                } else {
                    Trie::Node {
                        value,
                        left: Box::new(trie_insert(*left, key.skip(1), v)),
                        right,
                    }
                }
            }
        }
    }
}

} // verus!