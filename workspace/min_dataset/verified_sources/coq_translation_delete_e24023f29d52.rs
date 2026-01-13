use vstd::prelude::*;

verus! {

pub open spec fn tree_min(t: Tree) -> (nat, nat)
    recommends !matches!(t, Tree::E)
    decreases t
{
    match t {
        Tree::E => (0, 0),  // Should not happen
        Tree::T { left, key, value, right: _ } =>
            match *left {
                Tree::E => (key, value),
                _ => tree_min(*left),
            }
    }
}

pub open spec fn remove_min(t: Tree) -> Tree
    recommends !matches!(t, Tree::E)
    decreases t
{
    match t {
        Tree::E => Tree::E,
        Tree::T { left, key, value, right } =>
            match *left {
                Tree::E => *right,
                _ => Tree::T {
                    left: Box::new(remove_min(*left)),
                    key,
                    value,
                    right,
                },
            }
    }
}


pub enum Tree {
    E,
    T { left: Box<Tree>, key: nat, value: nat, right: Box<Tree> },
}


pub open spec fn delete(k: nat, t: Tree) -> Tree
    decreases t
{
    match t {
        Tree::E => Tree::E,
        Tree::T { left, key, value, right } =>
            if k < key {
                Tree::T {
                    left: Box::new(delete(k, *left)),
                    key,
                    value,
                    right,
                }
            } else if k > key {
                Tree::T {
                    left,
                    key,
                    value,
                    right: Box::new(delete(k, *right)),
                }
            } else {
                // Found the key to delete
                match (*left, *right) {
                    (Tree::E, Tree::E) => Tree::E,
                    (Tree::E, r) => r,
                    (l, Tree::E) => l,
                    (l, r) => {
                        // Both children exist: replace with minimum of right subtree
                        let (min_k, min_v) = tree_min(r);
                        Tree::T {
                            left: Box::new(l),
                            key: min_k,
                            value: min_v,
                            right: Box::new(remove_min(r)),
                        }
                    }
                }
            }
    }
}

} // verus!