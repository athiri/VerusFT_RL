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


pub open spec fn insert<V>(t: Trie<V>, key: Seq<bool>, v: V) -> Trie<V>
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
                        right: Box::new(insert(Trie::Leaf, key.skip(1), v)),
                    }
                } else {
                    Trie::Node {
                        value: None,
                        left: Box::new(insert(Trie::Leaf, key.skip(1), v)),
                        right: Box::new(Trie::Leaf),
                    }
                }
            }
            Trie::Node { value, left, right } => {
                if key[0] {
                    Trie::Node {
                        value,
                        left,
                        right: Box::new(insert(*right, key.skip(1), v)),
                    }
                } else {
                    Trie::Node {
                        value,
                        left: Box::new(insert(*left, key.skip(1), v)),
                        right,
                    }
                }
            }
        }
    }
}

} // verus!