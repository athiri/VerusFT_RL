use vstd::prelude::*;

verus! {

pub enum BTree {
    Leaf,
    Node { left: Box<BTree>, value: nat, right: Box<BTree> },
}

pub open spec fn btree_size(t: BTree) -> nat
    decreases t
{
    match t {
        BTree::Leaf => 0,
        BTree::Node { left, value: _, right } => 1 + btree_size(*left) + btree_size(*right),
    }
}

pub open spec fn btree_in_order(t: BTree) -> Seq<nat>
    decreases t
{
    match t {
        BTree::Leaf => Seq::empty(),
        BTree::Node { left, value, right } => {
            btree_in_order(*left).push(value).add(btree_in_order(*right))
        }
    }
}

pub proof fn btree_size_inorder(t: BTree)
    ensures btree_size(t) == btree_in_order(t).len()
    decreases t
{
    match t {
        BTree::Leaf => {}
        BTree::Node { left, value: _, right } => {
            btree_size_inorder(*left);
            btree_size_inorder(*right);
        }
    }
}

} // verus!
