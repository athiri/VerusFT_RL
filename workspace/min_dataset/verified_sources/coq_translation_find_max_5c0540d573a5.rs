use vstd::prelude::*;

verus! {

pub enum Tree {
    E,
    T { left: Box<Tree>, key: nat, value: nat, right: Box<Tree> },
}


pub open spec fn find_max(t: Tree) -> Option<nat>
    decreases t
{
    match t {
        Tree::E => None,
        Tree::T { left: _, key, value: _, right } =>
            match *right {
                Tree::E => Some(key),
                _ => find_max(*right),
            }
    }
}

} // verus!