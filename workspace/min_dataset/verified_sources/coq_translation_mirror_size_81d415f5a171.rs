use vstd::prelude::*;

verus! {

pub enum Tree { E, T { left: Box<Tree>, value: nat, right: Box<Tree> } }

pub open spec fn tree_size(t: Tree) -> nat decreases t {
    match t { Tree::E => 0, Tree::T { left, right, .. } => 1 + tree_size(*left) + tree_size(*right) }
}

pub open spec fn mirror(t: Tree) -> Tree decreases t {
    match t {
        Tree::E => Tree::E,
        Tree::T { left, value, right } => Tree::T { left: Box::new(mirror(*right)), value, right: Box::new(mirror(*left)) }
    }
}


pub proof fn mirror_size(t: Tree) ensures tree_size(mirror(t)) == tree_size(t) decreases t {
    reveal_with_fuel(mirror, 2); reveal_with_fuel(tree_size, 2);
    match t { Tree::E => {} Tree::T { left, right, .. } => { mirror_size(*left); mirror_size(*right); } }
}

} // verus!