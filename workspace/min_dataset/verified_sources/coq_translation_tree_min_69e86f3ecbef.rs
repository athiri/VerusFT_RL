use vstd::prelude::*;

verus! {

pub enum Tree {
    E,
    T { left: Box<Tree>, key: nat, value: nat, right: Box<Tree> },
}


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

} // verus!