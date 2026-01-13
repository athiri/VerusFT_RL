use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn is_divisible (n : int , divisor : int) -> bool { (n % divisor) == 0 }
fn is_non_prime (n : u64) -> (result : bool) requires n >= 2 , ensures result == (exists | k : int | 2 <= k < n && is_divisible (n as int , k)) , { let mut k = 2 ; while k < n invariant 2 <= k <= n , forall | j : int | 2 <= j < k ==> ! is_divisible (n as int , j) , decreases n - k { if n % k == 0 { assert (is_divisible (n as int , k as int)) ; assert (2 <= k < n) ; return true ; } k = k + 1 ; } assert (k == n) ; assert (forall | j : int | 2 <= j < n ==> ! is_divisible (n as int , j)) ; return false ; }

} // verus!