use vstd::prelude::*;

verus! {

pub enum Tree {
    E,
    T { left: Box<Tree>, key: nat, value: nat, right: Box<Tree> },
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

} // verus!