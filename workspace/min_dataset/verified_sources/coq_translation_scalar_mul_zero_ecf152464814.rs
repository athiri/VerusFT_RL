use vstd::prelude::*;

verus! {

pub open spec fn scalar_mul(k: nat, v: Seq<nat>) -> Seq<nat>
{ Seq::new(v.len(), |i: int| k * v[i]) }


pub proof fn scalar_mul_zero(v: Seq<nat>) ensures scalar_mul(0, v) =~= Seq::new(v.len(), |_i: int| 0nat) {}

} // verus!