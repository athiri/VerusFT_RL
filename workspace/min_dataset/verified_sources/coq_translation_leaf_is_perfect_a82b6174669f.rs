use vstd::prelude::*;

verus! {

pub open spec fn tree_height<T>(t: Tree<T>) -> nat
    decreases t
{
    match t {
        Tree::Leaf => 0,
        Tree::Node { left, value: _, right } => {
            let lh = tree_height(*left);
            let rh = tree_height(*right);
            1 + if lh > rh { lh } else { rh }
        }
    }
}


pub open spec fn is_perfect<T>(t: Tree<T>) -> bool
    decreases t
{
    match t {
        Tree::Leaf => true,
        Tree::Node { left, value: _, right } =>
            tree_height(*left) == tree_height(*right) &&
            is_perfect(*left) &&
            is_perfect(*right),
    }
}

pub open spec fn bool_to_dec(b: bool) -> Dec {
    if b { Dec::Yes } else { Dec::No }
}


pub enum Tree<T> {
    Leaf,
    Node { left: Box<Tree<T>>, value: T, right: Box<Tree<T>> },
}

pub enum Dec {
    Yes,
    No,
}

pub open spec fn dec_to_bool(d: Dec) -> bool {
    match d {
        Dec::Yes => true,
        Dec::No => false,
    }
}

pub open spec fn dec_is_perfect<T>(t: Tree<T>) -> Dec {
    bool_to_dec(is_perfect(t))
}


pub proof fn leaf_is_perfect()
    ensures dec_to_bool(dec_is_perfect::<nat>(Tree::Leaf))
{
}

} // verus!