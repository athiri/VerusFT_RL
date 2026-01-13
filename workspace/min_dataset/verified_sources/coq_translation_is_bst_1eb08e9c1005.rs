use vstd::prelude::*;

verus! {

pub open spec fn forall_tree(t: Tree, p: spec_fn(nat) -> bool) -> bool
    decreases t
{
    match t {
        Tree::E => true,
        Tree::T { left, key, value: _, right } =>
            p(key) && forall_tree(*left, p) && forall_tree(*right, p),
    }
}


pub enum Tree {
    E,
    T { left: Box<Tree>, key: nat, value: nat, right: Box<Tree> },
}

pub open spec fn all_lt(t: Tree, bound: nat) -> bool {
    forall_tree(t, |k: nat| k < bound)
}

pub open spec fn all_gt(t: Tree, bound: nat) -> bool {
    forall_tree(t, |k: nat| k > bound)
}


pub open spec fn is_bst(t: Tree) -> bool
    decreases t
{
    match t {
        Tree::E => true,
        Tree::T { left, key, value: _, right } =>
            all_lt(*left, key) &&
            all_gt(*right, key) &&
            is_bst(*left) &&
            is_bst(*right),
    }
}

} // verus!