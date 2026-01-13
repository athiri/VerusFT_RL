use vstd::prelude::*;

verus! {

pub enum Tree { E, T { left: Box<Tree>, value: nat, right: Box<Tree> } }

pub open spec fn tree_size(t: Tree) -> nat decreases t {
    match t { Tree::E => 0, Tree::T { left, right, .. } => 1 + tree_size(*left) + tree_size(*right) }
}

pub open spec fn flatten(t: Tree) -> Seq<nat> decreases t {
    match t {
        Tree::E => Seq::empty(),
        Tree::T { left, value, right } => flatten(*left) + seq![value] + flatten(*right)
    }
}


pub proof fn flatten_len(t: Tree) ensures flatten(t).len() == tree_size(t) decreases t {
    reveal_with_fuel(flatten, 2); reveal_with_fuel(tree_size, 2);
    match t { Tree::E => {} Tree::T { left, right, .. } => { flatten_len(*left); flatten_len(*right); } }
}

} // verus!