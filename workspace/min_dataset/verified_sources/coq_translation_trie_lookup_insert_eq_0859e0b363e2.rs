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


pub proof fn trie_lookup_insert_eq<V>(t: Trie<V>, key: Seq<bool>, v: V)
    ensures trie_lookup(trie_insert(t, key, v), key) == Some(v)
    decreases key.len()
{
    reveal_with_fuel(trie_lookup, 3);
    reveal_with_fuel(trie_insert, 3);
    if key.len() == 0 {
        // Base case: empty key
    } else {
        match t {
            Trie::Leaf => {
                if key[0] {
                    trie_lookup_insert_eq(Trie::Leaf, key.skip(1), v);
                } else {
                    trie_lookup_insert_eq(Trie::Leaf, key.skip(1), v);
                }
            }
            Trie::Node { value: _, left, right } => {
                if key[0] {
                    trie_lookup_insert_eq(*right, key.skip(1), v);
                } else {
                    trie_lookup_insert_eq(*left, key.skip(1), v);
                }
            }
        }
    }
}

} // verus!