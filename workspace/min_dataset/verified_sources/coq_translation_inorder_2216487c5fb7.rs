use vstd::prelude::*;

verus! {

pub enum Tree {
    Leaf,
    Node { left: Box<Tree>, value: nat, right: Box<Tree> },
}


pub open spec fn inorder(t: Tree) -> Seq<nat>
    decreases t
{
    match t {
        Tree::Leaf => Seq::empty(),
        Tree::Node { left, value, right } =>
            inorder(*left) + seq![value] + inorder(*right),
    }
}

} // verus!