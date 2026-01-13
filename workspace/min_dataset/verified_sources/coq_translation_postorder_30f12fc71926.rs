use vstd::prelude::*;

verus! {

pub enum Tree {
    Leaf,
    Node { left: Box<Tree>, value: nat, right: Box<Tree> },
}


pub open spec fn postorder(t: Tree) -> Seq<nat>
    decreases t
{
    match t {
        Tree::Leaf => Seq::empty(),
        Tree::Node { left, value, right } =>
            postorder(*left) + postorder(*right) + seq![value],
    }
}

} // verus!