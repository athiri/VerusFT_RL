use vstd::prelude::*;

verus! {

pub enum Tree { E, T { left: Box<Tree>, value: nat, right: Box<Tree> } }

pub open spec fn mirror(t: Tree) -> Tree decreases t {
    match t {
        Tree::E => Tree::E,
        Tree::T { left, value, right } => Tree::T { left: Box::new(mirror(*right)), value, right: Box::new(mirror(*left)) }
    }
}

pub open spec fn tree_height(t: Tree) -> nat decreases t {
    match t { Tree::E => 0, Tree::T { left, right, .. } => 1 + if tree_height(*left) > tree_height(*right) { tree_height(*left) } else { tree_height(*right) } }
}


pub proof fn mirror_height(t: Tree) ensures tree_height(mirror(t)) == tree_height(t) decreases t {
    reveal_with_fuel(mirror, 2); reveal_with_fuel(tree_height, 2);
    match t { Tree::E => {} Tree::T { left, right, .. } => { mirror_height(*left); mirror_height(*right); } }
}

} // verus!