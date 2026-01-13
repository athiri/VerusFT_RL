use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn is_prime_pred (n : u32) -> bool { forall | k : int | 2 <= k < n ==> # [trigger] (n as int % k) != 0 }
# [verifier :: loop_isolation (false)] fn is_prime (n : u32) -> (result : bool) requires n >= 2 , ensures result ==> (forall | k : int | 2 <= k < n ==> # [trigger] (n as int % k) != 0) , ! result ==> exists | k : int | 2 <= k < n && # [trigger] (n as int % k) == 0 , { let mut i = 2u32 ; while i < n invariant 2 <= i <= n , forall | k : int | 2 <= k < i ==> # [trigger] (n as int % k) != 0 , decreases n - i { if n % i == 0 { return false ; } i = i + 1 ; } true }
# [verifier :: loop_isolation (false)] fn largest_prime_factor (n : u32) -> (result : u32) requires 2 <= n <= MAX - 1 , ensures 1 <= result <= n , result == 1 || (result > 1 && is_prime_pred (result)) { let mut largest = 1u32 ; let mut i = 2u32 ; while i <= n invariant 2 <= i <= n + 1 , 1 <= largest <= n , largest == 1 || (largest > 1 && is_prime_pred (largest)) , decreases n + 1 - i { if n % i == 0 && is_prime (i) { largest = i ; } i = i + 1 ; } largest }

} // verus!