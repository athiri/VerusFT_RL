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


pub proof fn lookup_insert_eq(d: nat, k: nat, v: nat, t: Tree)
    ensures lookup(d, k, insert(k, v, t)) == v
    decreases t
{
    match t {
        Tree::E => {}
        Tree::T { left, key, value: _, right } => {
            if k < key {
                lookup_insert_eq(d, k, v, *left);
            } else if k > key {
                lookup_insert_eq(d, k, v, *right);
            } else {
            }
        }
    }
}

} // verus!