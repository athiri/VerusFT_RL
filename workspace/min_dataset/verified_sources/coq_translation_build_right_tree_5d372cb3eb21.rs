use vstd::prelude::*;

verus! {

pub enum Tree {
    Leaf,
    Node { left: Box<Tree>, value: nat, right: Box<Tree> },
}


pub open spec fn build_right_tree(s: Seq<nat>) -> Tree
    decreases s.len()
{
    if s.len() == 0 {
        Tree::Leaf
    } else {
        Tree::Node {
            left: Box::new(Tree::Leaf),
            value: s[0],
            right: Box::new(build_right_tree(s.skip(1))),
        }
    }
}

} // verus!