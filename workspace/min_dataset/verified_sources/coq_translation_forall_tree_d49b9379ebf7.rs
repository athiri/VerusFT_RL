use vstd::prelude::*;

verus! {

pub enum Tree {
    E,
    T { left: Box<Tree>, key: nat, value: nat, right: Box<Tree> },
}


pub open spec fn forall_tree(t: Tree, p: spec_fn(nat) -> bool) -> bool
    decreases t
{
    match t {
        Tree::E => true,
        Tree::T { left, key, value: _, right } =>
            p(key) && forall_tree(*left, p) && forall_tree(*right, p),
    }
}

} // verus!