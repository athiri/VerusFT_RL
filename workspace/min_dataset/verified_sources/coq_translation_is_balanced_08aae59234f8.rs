use vstd::prelude::*;

verus! {

pub open spec fn max(a: nat, b: nat) -> nat { if a > b { a } else { b } }


pub enum Tree { E, T { left: Box<Tree>, value: nat, right: Box<Tree> } }

pub open spec fn height(t: Tree) -> nat decreases t {
    match t { Tree::E => 0, Tree::T { left, right, .. } => 1 + max(height(*left), height(*right)) }
}

pub open spec fn abs_diff(a: nat, b: nat) -> nat { if a > b { (a - b) as nat } else { (b - a) as nat } }


pub open spec fn is_balanced(t: Tree) -> bool decreases t {
    match t {
        Tree::E => true,
        Tree::T { left, right, .. } =>
            abs_diff(height(*left), height(*right)) <= 1 && is_balanced(*left) && is_balanced(*right)
    }
}

} // verus!