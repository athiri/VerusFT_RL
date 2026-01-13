use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn selection_sort (a : & mut Vec < int >) ensures a . len () == old (a) . len () , forall | i : int , j : int | 0 <= i < j < a . len () ==> a [i] <= a [j] , { let n = a . len () ; let mut i = 0 ; while i < n invariant a . len () == n , forall | p : int , q : int | 0 <= p < q < i ==> a [p] <= a [q] , forall | p : int , q : int | 0 <= p < i <= q < n ==> a [p] <= a [q] , decreases n - i { let mut min_idx = i ; let mut j = i + 1 ; while j < n invariant a . len () == n , i <= min_idx < n , i < j <= n , forall | k : int | i as int <= k < j as int ==> a [min_idx as int] <= a [k] , forall | p : int , q : int | 0 <= p < q < i ==> a [p] <= a [q] , forall | p : int , q : int | 0 <= p < i <= q < n ==> a [p] <= a [q] , decreases n - j { if a [j] < a [min_idx] { min_idx = j ; } j += 1 ; } let temp = a [i] ; let min_val = a [min_idx] ; a . set (i , min_val) ; a . set (min_idx , temp) ; i += 1 ; } }

} // verus!