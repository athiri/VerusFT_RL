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


pub open spec fn all_lt(t: Tree, bound: nat) -> bool {
    forall_tree(t, |k: nat| k < bound)
}

pub open spec fn all_gt(t: Tree, bound: nat) -> bool {
    forall_tree(t, |k: nat| k > bound)
}


pub enum Tree {
    E,
    T { left: Box<Tree>, key: nat, value: nat, right: Box<Tree> },
}

pub open spec fn contains(k: nat, t: Tree) -> bool
    decreases t
{
    match t {
        Tree::E => false,
        Tree::T { left, key, value: _, right } =>
            if k < key {
                contains(k, *left)
            } else if k > key {
                contains(k, *right)
            } else {
                true
            }
    }
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

pub open spec fn lookup(d: nat, k: nat, t: Tree) -> nat
    decreases t
{
    match t {
        Tree::E => d,
        Tree::T { left, key, value, right } =>
            if k < key {
                lookup(d, k, *left)
            } else if k > key {
                lookup(d, k, *right)
            } else {
                value
            }
    }
}


pub proof fn not_contains_lookup(d: nat, k: nat, t: Tree)
    requires is_bst(t), !contains(k, t)
    ensures lookup(d, k, t) == d
    decreases t
{
    match t {
        Tree::E => {}
        Tree::T { left, key, value: _, right } => {
            if k < key {
                not_contains_lookup(d, k, *left);
            } else if k > key {
                not_contains_lookup(d, k, *right);
            }
        }
    }
}

} // verus!