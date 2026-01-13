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

pub open spec fn contains<V>(t: Trie<V>, key: Seq<bool>) -> bool {
    lookup(t, key).is_some()
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


pub proof fn remove_not_contains<V>(t: Trie<V>, key: Seq<bool>)
    ensures !contains(remove(t, key), key)
    decreases key.len()
{
    reveal_with_fuel(lookup, 3);
    reveal_with_fuel(remove, 3);
    match t {
        Trie::Leaf => {}
        Trie::Node { value: _, left, right } => {
            if key.len() > 0 {
                if key[0] {
                    remove_not_contains(*right, key.skip(1));
                } else {
                    remove_not_contains(*left, key.skip(1));
                }
            }
        }
    }
}

} // verus!