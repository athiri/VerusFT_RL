use vstd::prelude::*;

verus! {

pub enum Tree { E, T { left: Box<Tree>, value: nat, right: Box<Tree> } }


pub open spec fn tree_height(t: Tree) -> nat decreases t {
    match t { Tree::E => 0, Tree::T { left, right, .. } => 1 + if tree_height(*left) > tree_height(*right) { tree_height(*left) } else { tree_height(*right) } }
}

} // verus!