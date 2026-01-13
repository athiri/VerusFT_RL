use vstd::prelude::*;

verus! {

pub open spec fn gen_pure<A>(a: A, _seed: nat, _size: nat) -> A {
    a
}


pub proof fn gen_pure_law<A>(a: A, seed: nat, size: nat)
    ensures gen_pure(a, seed, size) == a
{
    // Trivially true
}

} // verus!