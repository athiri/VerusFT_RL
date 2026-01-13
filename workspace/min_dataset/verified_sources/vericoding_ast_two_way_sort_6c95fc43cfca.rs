use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
# [verifier :: external_body] fn swap (a : & mut Vec < bool > , i : usize , j : usize) requires 0 <= i < j < old (a) . len () , ensures a [i as int] == old (a) [j as int] , a [j as int] == old (a) [i as int] , forall | k : int | 0 <= k < a . len () && k != i && k != j ==> a [k] == old (a) [k] , a . len () == old (a) . len () , a @ . to_multiset () =~~= old (a) @ . to_multiset () , { }
# [verifier :: loop_isolation (false)] fn two_way_sort (a : & mut Vec < bool >) requires old (a) . len () <= 100_000 , ensures a . len () == old (a) . len () , a @ . to_multiset () == old (a) @ . to_multiset () , forall | i : int , j : int | 0 <= i < j < a . len () ==> ! a [i] || a [j] , { let mut left = 0 ; let mut right = a . len () ; while left < right invariant 0 <= left <= right <= a . len () , a . len () == old (a) . len () , a @ . to_multiset () == old (a) @ . to_multiset () , forall | k : int | 0 <= k < left ==> ! a [k] , forall | k : int | right <= k < a . len () ==> a [k] , decreases right - left { if left < a . len () && ! a [left] { left += 1 ; } else if right > 0 && a [right - 1] { right -= 1 ; } else if left < right - 1 { swap (a , left , right - 1) ; left += 1 ; right -= 1 ; } else { break ; } } }

} // verus!