use vstd::prelude::*;

verus! {

pub enum Tree {
    Leaf,
    Node { left: Box<Tree>, value: nat, right: Box<Tree> },
}


pub open spec fn level_order_level(t: Tree, level: nat) -> Seq<nat>
    decreases t, level
{
    match t {
        Tree::Leaf => Seq::empty(),
        Tree::Node { left, value, right } =>
            if level == 0 {
                seq![value]
            } else {
                level_order_level(*left, (level - 1) as nat) +
                level_order_level(*right, (level - 1) as nat)
            }
    }
}

} // verus!