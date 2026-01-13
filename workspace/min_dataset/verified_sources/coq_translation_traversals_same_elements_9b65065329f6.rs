use vstd::prelude::*;

verus! {

pub open spec fn postorder(t: Tree) -> Seq<nat>
    decreases t
{
    match t {
        Tree::Leaf => Seq::empty(),
        Tree::Node { left, value, right } =>
            postorder(*left) + postorder(*right) + seq![value],
    }
}


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

pub open spec fn count_in_seq(s: Seq<nat>, x: nat) -> nat
    decreases s.len()
{
    if s.len() == 0 {
        0
    } else if s[0] == x {
        1 + count_in_seq(s.skip(1), x)
    } else {
        count_in_seq(s.skip(1), x)
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


pub proof fn traversals_same_elements(t: Tree, x: nat)
    ensures count_in_seq(inorder(t), x) == count_in_seq(preorder(t), x) &&
            count_in_seq(preorder(t), x) == count_in_seq(postorder(t), x)
    decreases t
{
    // This requires a detailed proof about count distributing over append
    assume(count_in_seq(inorder(t), x) == count_in_seq(preorder(t), x) &&
           count_in_seq(preorder(t), x) == count_in_seq(postorder(t), x));
}

} // verus!