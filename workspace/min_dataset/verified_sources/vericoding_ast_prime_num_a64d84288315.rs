use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn is_divisible (n : int , divisor : int) -> bool { (n % divisor) == 0 }
fn prime_num (n : u64) -> (result : bool) requires n >= 2 , ensures result == (forall | k : int | 2 <= k < n ==> ! is_divisible (n as int , k)) , { let mut i : u64 = 2 ; while i < n invariant 2 <= i <= n , forall | k : int | 2 <= k < i ==> ! is_divisible (n as int , k) , decreases n - i , { if n % i == 0 { assert (is_divisible (n as int , i as int)) ; assert (2 <= i < n) ; assert (exists | k : int | 2 <= k < n && is_divisible (n as int , k)) ; assert (! (forall | k : int | 2 <= k < n ==> ! is_divisible (n as int , k))) ; return false ; } i = i + 1 ; } assert (i == n) ; assert (forall | k : int | 2 <= k < n ==> ! is_divisible (n as int , k)) ; return true ; }

} // verus!