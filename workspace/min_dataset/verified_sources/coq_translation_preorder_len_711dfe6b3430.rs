use vstd::prelude::*;

verus! {

pub enum Tree {
    Leaf,
    Node { left: Box<Tree>, value: nat, right: Box<Tree> },
}

pub open spec fn tree_size(t: Tree) -> nat
    decreases t
{
    match t {
        Tree::Leaf => 0,
        Tree::Node { left, value: _, right } =>
            1 + tree_size(*left) + tree_size(*right),
    }
}

pub open spec fn preorder(t: Tree) -> Seq<nat>
    decreases t
{
    match t {
        Tree::Leaf => Seq::empty(),
        Tree::Node { left, value, right } =>
            seq![value] + preorder(*left) + preorder(*right),
    }
}


pub proof fn preorder_len(t: Tree)
    ensures preorder(t).len() == tree_size(t)
    decreases t
{
    reveal_with_fuel(preorder, 2);
    reveal_with_fuel(tree_size, 2);
    match t {
        Tree::Leaf => {}
        Tree::Node { left, value: _, right } => {
            preorder_len(*left);
            preorder_len(*right);
        }
    }
}

} // verus!