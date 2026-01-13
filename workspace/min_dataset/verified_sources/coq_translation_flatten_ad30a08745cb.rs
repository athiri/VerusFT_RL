use vstd::prelude::*;

verus! {

pub enum Tree { E, T { left: Box<Tree>, value: nat, right: Box<Tree> } }


pub open spec fn flatten(t: Tree) -> Seq<nat> decreases t {
    match t {
        Tree::E => Seq::empty(),
        Tree::T { left, value, right } => flatten(*left) + seq![value] + flatten(*right)
    }
}

} // verus!