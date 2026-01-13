use vstd::prelude::*;

verus! {

pub enum Tree { E, T { left: Box<Tree>, value: nat, right: Box<Tree> } }

pub open spec fn mirror(t: Tree) -> Tree decreases t {
    match t {
        Tree::E => Tree::E,
        Tree::T { left, value, right } => Tree::T { left: Box::new(mirror(*right)), value, right: Box::new(mirror(*left)) }
    }
}


pub proof fn mirror_mirror(t: Tree) ensures mirror(mirror(t)) == t decreases t {
    reveal_with_fuel(mirror, 2);
    match t { Tree::E => {} Tree::T { left, right, .. } => { mirror_mirror(*left); mirror_mirror(*right); } }
}

} // verus!