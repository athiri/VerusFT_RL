use vstd::prelude::*;

verus! {

pub enum Tree {
    E,
    T { left: Box<Tree>, key: nat, value: nat, right: Box<Tree> },
}


pub open spec fn find_min(t: Tree) -> Option<nat>
    decreases t
{
    match t {
        Tree::E => None,
        Tree::T { left, key, value: _, right: _ } =>
            match *left {
                Tree::E => Some(key),
                _ => find_min(*left),
            }
    }
}

} // verus!