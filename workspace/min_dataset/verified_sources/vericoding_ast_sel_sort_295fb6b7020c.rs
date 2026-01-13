use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn sorted_seg (a : Seq < int > , i : int , j : int) -> bool recommends 0 <= i <= j <= a . len () { forall | l : int , k : int | i <= l <= k < j ==> a [l] <= a [k] }
fn sel_sort (a : & mut Vec < int > , c : usize , f : usize) requires c <= f , f <= old (a) . len () , ensures a . len () == old (a) . len () , sorted_seg (a @ , c as int , f as int) , { let mut i = c ; while i < f invariant c <= i <= f , f <= a . len () , a . len () == old (a) . len () , sorted_seg (a @ , c as int , i as int) , forall | k : int , l : int | c <= k < i && i <= l < f ==> a @ [k] <= a @ [l] , decreases f - i { let mut min_idx = i ; let mut j = i + 1 ; while j < f invariant i < f , i <= min_idx < j <= f , f <= a . len () , a . len () == old (a) . len () , forall | k : int | i <= k < j ==> a @ [min_idx as int] <= a @ [k] , decreases f - j { if a [j] < a [min_idx] { min_idx = j ; } j += 1 ; } let temp = a [i] ; let min_val = a [min_idx] ; a . set (i , min_val) ; a . set (min_idx , temp) ; i += 1 ; } }

} // verus!