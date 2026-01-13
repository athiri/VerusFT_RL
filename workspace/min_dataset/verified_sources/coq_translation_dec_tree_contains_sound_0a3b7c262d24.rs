use vstd::prelude::*;

verus! {

pub open spec fn bool_to_dec(b: bool) -> Dec {
    if b { Dec::Yes } else { Dec::No }
}


pub enum Tree<T> {
    Leaf,
    Node { left: Box<Tree<T>>, value: T, right: Box<Tree<T>> },
}

pub open spec fn tree_contains<T>(t: Tree<T>, x: T, eq: spec_fn(T, T) -> bool) -> bool
    decreases t
{
    match t {
        Tree::Leaf => false,
        Tree::Node { left, value, right } =>
            eq(value, x) ||
            tree_contains(*left, x, eq) ||
            tree_contains(*right, x, eq),
    }
}

pub enum Dec {
    Yes,
    No,
}

pub open spec fn dec_tree_contains<T>(t: Tree<T>, x: T, eq: spec_fn(T, T) -> bool) -> Dec {
    bool_to_dec(tree_contains(t, x, eq))
}

pub open spec fn dec_to_bool(d: Dec) -> bool {
    match d {
        Dec::Yes => true,
        Dec::No => false,
    }
}


pub proof fn dec_tree_contains_sound<T>(t: Tree<T>, x: T, eq: spec_fn(T, T) -> bool)
    requires forall|a: T, b: T| #[trigger] eq(a, b) <==> (a == b)
    ensures dec_to_bool(dec_tree_contains(t, x, eq)) <==> tree_contains(t, x, eq)
{
}

} // verus!