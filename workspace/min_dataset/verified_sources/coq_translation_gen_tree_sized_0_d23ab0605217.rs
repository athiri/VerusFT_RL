use vstd::prelude::*;

verus! {

pub open spec fn all_values_bounded(t: Tree<nat>, bound: nat) -> bool
    decreases t
{
    match t {
        Tree::Leaf => true,
        Tree::Node { left, value, right } =>
            value <= bound &&
            all_values_bounded(*left, bound) &&
            all_values_bounded(*right, bound),
    }
}


pub open spec fn gen_tree_sized(size: nat) -> Set<Tree<nat>> {
    Set::new(|t: Tree<nat>|
        tree_height(t) <= size &&
        all_values_bounded(t, size)
    )
}


pub enum Tree<T> {
    Leaf,
    Node { left: Box<Tree<T>>, value: T, right: Box<Tree<T>> },
}

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


pub proof fn gen_tree_sized_0()
    ensures forall|t: Tree<nat>| #[trigger] gen_tree_sized(0nat).contains(t) ==> t == Tree::<nat>::Leaf
{
    assert forall|t: Tree<nat>| #[trigger] gen_tree_sized(0nat).contains(t) implies t == Tree::<nat>::Leaf by {
        reveal_with_fuel(tree_height, 2);
        if tree_height(t) == 0 {
            match t {
                Tree::Leaf => {}
                Tree::Node { left: _, value: _, right: _ } => {
                    assert(false);  // Height would be > 0
                }
            }
        }
    }
}

} // verus!