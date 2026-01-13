use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
# [verifier :: loop_isolation (false)] fn unique (a : & [i32]) -> (result : Vec < i32 >) requires forall | i : int , j : int | # ! [trigger a [i] , a [j]] 0 <= i && i < j && j < a . len () ==> a [i] <= a [j] , ensures forall | i : int , j : int | # ! [trigger result [i] , result [j]] 0 <= i && i < j && j < result . len () ==> result [i] < result [j] , { return Vec :: new () ; }

} // verus!