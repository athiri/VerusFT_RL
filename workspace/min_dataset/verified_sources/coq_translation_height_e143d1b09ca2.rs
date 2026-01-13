use vstd::prelude::*;

verus! {

pub enum Tree { E, T { left: Box<Tree>, value: nat, right: Box<Tree> } }

pub open spec fn max(a: nat, b: nat) -> nat { if a > b { a } else { b } }


pub open spec fn height(t: Tree) -> nat decreases t {
    match t { Tree::E => 0, Tree::T { left, right, .. } => 1 + max(height(*left), height(*right)) }
}

} // verus!