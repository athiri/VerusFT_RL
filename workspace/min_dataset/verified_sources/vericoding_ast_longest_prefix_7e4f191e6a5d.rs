use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn seq_equal_prefix (a : Seq < i32 > , b : Seq < i32 > , len : int) -> bool { forall | k : int | 0 <= k < len ==> a [k] == b [k] }
fn longest_prefix (a : & [i32] , b : & [i32]) -> (i : usize) ensures i <= a . len () && i <= b . len () , seq_equal_prefix (a @ , b @ , i as int) , i < a . len () && i < b . len () ==> a @ [i as int] != b @ [i as int] { let mut i : usize = 0 ; while i < a . len () && i < b . len () && a [i] == b [i] invariant i <= a . len () && i <= b . len () , seq_equal_prefix (a @ , b @ , i as int) decreases a . len () - i { i = i + 1 ; } i }

} // verus!