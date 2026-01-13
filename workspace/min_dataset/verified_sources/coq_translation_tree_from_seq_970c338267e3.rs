use vstd::prelude::*;

verus! {

pub enum Tree { E, T { left: Box<Tree>, value: nat, right: Box<Tree> } }


pub open spec fn tree_from_seq(s: Seq<nat>) -> Tree decreases s.len() {
    if s.len() == 0 { Tree::E }
    else { Tree::T { left: Box::new(Tree::E), value: s[0], right: Box::new(tree_from_seq(s.skip(1))) } }
}

} // verus!