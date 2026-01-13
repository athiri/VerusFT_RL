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

pub open spec fn all_gt(t: Tree, bound: nat) -> bool {
    forall_tree(t, |k: nat| k > bound)
}

pub open spec fn insert(k: nat, v: nat, t: Tree) -> Tree
    decreases t
{
    match t {
        Tree::E => Tree::T {
            left: Box::new(Tree::E),
            key: k,
            value: v,
            right: Box::new(Tree::E),
        },
        Tree::T { left, key, value, right } =>
            if k < key {
                Tree::T {
                    left: Box::new(insert(k, v, *left)),
                    key,
                    value,
                    right,
                }
            } else if k > key {
                Tree::T {
                    left,
                    key,
                    value,
                    right: Box::new(insert(k, v, *right)),
                }
            } else {
                Tree::T { left, key: k, value: v, right }
            }
    }
}


pub proof fn insert_all_gt(k: nat, v: nat, t: Tree, bound: nat)
    requires all_gt(t, bound), k > bound
    ensures all_gt(insert(k, v, t), bound)
    decreases t
{
    reveal_with_fuel(forall_tree, 3);
    match t {
        Tree::E => {}
        Tree::T { left, key, value: _, right } => {
            if k < key {
                insert_all_gt(k, v, *left, bound);
            } else if k > key {
                insert_all_gt(k, v, *right, bound);
            }
        }
    }
}

} // verus!