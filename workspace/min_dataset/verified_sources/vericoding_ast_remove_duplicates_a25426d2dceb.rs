use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn in_array (a : Seq < i32 > , x : i32) -> (ret : bool) { exists | i : int | 0 <= i < a . len () && a [i] == x }
# [verifier :: loop_isolation (false)] fn remove_duplicates (a : & [i32]) -> (result : Vec < i32 >) requires a . len () >= 1 , ensures forall | i : int | # ! [auto] 0 <= i < result . len () ==> in_array (a @ , result [i]) , forall | i : int , j : int | 0 <= i < j < result . len () ==> result [i] != result [j] , { let mut result = Vec :: new () ; for i in 0 .. a . len () invariant forall | k : int | # ! [auto] 0 <= k < result . len () ==> in_array (a @ , result [k]) , forall | k1 : int , k2 : int | 0 <= k1 < k2 < result . len () ==> result [k1] != result [k2] , { let mut found = false ; for j in 0 .. result . len () invariant forall | k : int | # ! [auto] 0 <= k < result . len () ==> in_array (a @ , result [k]) , forall | k1 : int , k2 : int | 0 <= k1 < k2 < result . len () ==> result [k1] != result [k2] , found == exists | k : int | 0 <= k < j && result [k] == a [i as int] , { if result [j] == a [i] { found = true ; break ; } } if ! found { result . push (a [i]) ; } } result }

} // verus!