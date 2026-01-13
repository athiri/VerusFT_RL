use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
proof fn lemma_vec_push < T > (vec : Seq < T > , i : T , l : usize) requires l == vec . len () , ensures forall | k : int | 0 <= k < vec . len () ==> # [trigger] vec [k] == vec . push (i) [k] , vec . push (i) . index (l as int) == i , { }
fn contains (str : & Vec < i32 > , key : i32) -> (result : bool) ensures result <==> (exists | i : int | 0 <= i < str . len () && (str [i] == key)) , { let mut idx = 0 ; while idx < str . len () invariant 0 <= idx <= str . len () , forall | j : int | 0 <= j < idx ==> str [j] != key , decreases str . len () - idx , { if str [idx] == key { return true ; } idx += 1 ; } false }
fn remove_elements (arr1 : & Vec < i32 > , arr2 : & Vec < i32 >) -> (result : Vec < i32 >) ensures forall | i : int | 0 <= i < result . len () ==> (arr1 @ . contains (# [trigger] result [i]) && ! arr2 @ . contains (# [trigger] result [i] ,)) , forall | i : int | 0 <= i < arr1 . len () ==> (arr2 @ . contains (# [trigger] arr1 [i]) || result @ . contains (# [trigger] arr1 [i] ,)) , { let mut result = Vec :: new () ; let mut idx = 0 ; while idx < arr1 . len () invariant 0 <= idx <= arr1 . len () , forall | i : int | 0 <= i < result . len () ==> (arr1 @ . contains (# [trigger] result [i]) && ! arr2 @ . contains (# [trigger] result [i] ,)) , forall | i : int | 0 <= i < idx ==> (arr2 @ . contains (# [trigger] arr1 [i]) || result @ . contains (# [trigger] arr1 [i] ,)) , decreases arr1 . len () - idx , { let elem = arr1 [idx] ; if ! contains (arr2 , elem) { proof { let old_result = result @ ; let old_len = result . len () ; lemma_vec_push (old_result , elem , old_len) ; } result . push (elem) ; } idx += 1 ; } result }

} // verus!