use vstd::prelude::*;

verus! {

pub enum Tree {
    Leaf,
    Node { left: Box<Tree>, right: Box<Tree> },
}


pub open spec fn shrink_tree(t: Tree) -> Seq<Tree>
    decreases t
{
    match t {
        Tree::Leaf => seq![],
        Tree::Node { left, right } => {
            seq![Tree::Leaf, *left, *right]
        }
    }
}

} // verus!