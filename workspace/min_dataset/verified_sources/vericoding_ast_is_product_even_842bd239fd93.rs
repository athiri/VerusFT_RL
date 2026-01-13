use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn is_even (n : u32) -> bool { (n % 2) == 0 }
fn is_product_even (arr : & Vec < u32 >) -> (result : bool) ensures result <==> (exists | k : int | 0 <= k < arr . len () && is_even (# [trigger] arr [k])) , { let mut i = 0 ; while i < arr . len () invariant 0 <= i <= arr . len () , forall | k : int | 0 <= k < i ==> ! is_even (# [trigger] arr [k]) , decreases arr . len () - i , { if arr [i] % 2 == 0 { assert (is_even (arr [i as int])) ; return true ; } assert (! is_even (arr [i as int])) ; i += 1 ; } false }

} // verus!