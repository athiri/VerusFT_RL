use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn append (v : & Vec < u64 > , elem : u64) -> (c : Vec < u64 >) requires v . len () <= 100 , ensures c @ . len () == v @ . len () + 1 , forall | i : int | (0 <= i && i < v . len ()) ==> c [i] == v [i] , c @ . last () == elem , { let mut result = v . clone () ; result . push (elem) ; result }

} // verus!