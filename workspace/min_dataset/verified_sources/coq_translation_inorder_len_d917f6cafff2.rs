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

pub open spec fn inorder(t: Tree) -> Seq<nat>
    decreases t
{
    match t {
        Tree::Leaf => Seq::empty(),
        Tree::Node { left, value, right } =>
            inorder(*left) + seq![value] + inorder(*right),
    }
}


pub proof fn inorder_len(t: Tree)
    ensures inorder(t).len() == tree_size(t)
    decreases t
{
    reveal_with_fuel(inorder, 2);
    reveal_with_fuel(tree_size, 2);
    match t {
        Tree::Leaf => {}
        Tree::Node { left, value: _, right } => {
            inorder_len(*left);
            inorder_len(*right);
        }
    }
}

} // verus!