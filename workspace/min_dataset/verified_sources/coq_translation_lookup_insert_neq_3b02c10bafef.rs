use vstd::prelude::*;

verus! {

pub enum Tree {
    E,  // Empty
    T { left: Box<Tree>, key: nat, value: nat, right: Box<Tree> },
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


pub proof fn lookup_insert_neq(d: nat, k1: nat, k2: nat, v: nat, t: Tree)
    requires k1 != k2
    ensures lookup(d, k1, insert(k2, v, t)) == lookup(d, k1, t)
    decreases t
{
    match t {
        Tree::E => {}
        Tree::T { left, key, value: _, right } => {
            if k2 < key {
                if k1 < key {
                    lookup_insert_neq(d, k1, k2, v, *left);
                }
            } else if k2 > key {
                if k1 > key {
                    lookup_insert_neq(d, k1, k2, v, *right);
                }
            }
        }
    }
}

} // verus!