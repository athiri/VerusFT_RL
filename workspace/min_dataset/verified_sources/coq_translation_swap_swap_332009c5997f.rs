use vstd::prelude::*;

verus! {

pub open spec fn swap<A, B>(p: (A, B)) -> (B, A) { (p.1, p.0) }


pub proof fn swap_swap<A, B>(p: (A, B)) ensures swap(swap(p)) == p {}

} // verus!