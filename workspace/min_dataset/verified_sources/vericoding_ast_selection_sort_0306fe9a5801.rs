use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn selection_sort (a : & mut Vec < int >) ensures a . len () == old (a) . len () , forall | i : int , j : int | 0 <= i < j < a . len () ==> a [i] <= a [j] , { let len = a . len () ; let mut i : usize = 0 ; while i < len invariant i <= len , a . len () == len , forall | x : int , y : int | 0 <= x < y < i ==> a [x] <= a [y] , forall | x : int , y : int | 0 <= x < i && i <= y < len ==> a [x] <= a [y] , decreases len - i { let mut min_idx : usize = i ; let mut j : usize = i + 1 ; while j < len invariant i <= min_idx < len , i <= j <= len , a . len () == len , forall | k : int | i <= k < j ==> a [min_idx as int] <= a [k] , decreases len - j { if a [j] < a [min_idx] { min_idx = j ; } j += 1 ; } let temp = a [i] ; let min_val = a [min_idx] ; a . set (i , min_val) ; a . set (min_idx , temp) ; i += 1 ; } }

} // verus!