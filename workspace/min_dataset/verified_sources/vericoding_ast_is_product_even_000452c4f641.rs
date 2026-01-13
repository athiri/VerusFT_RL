use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn is_even (n : u32) -> bool { (n % 2) == 0 }
fn is_even_exec (n : u32) -> (result : bool) ensures result == is_even (n) { (n % 2) == 0 }
fn is_product_even (arr : & Vec < u32 >) -> (result : bool) ensures result <==> (exists | k : int | 0 <= k < arr . len () && is_even (# [trigger] arr [k])) , { for i in 0 .. arr . len () invariant forall | k : int | 0 <= k < i ==> ! is_even (# [trigger] arr [k]) , { if is_even_exec (arr [i]) { return true ; } } false }

} // verus!