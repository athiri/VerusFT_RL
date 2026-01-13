use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn abs_spec (i : int) -> int { if i < 0 { - i } else { i } }
# [verifier :: loop_isolation (false)] fn has_close_elements (numbers : & [i32] , threshold : i32) -> (flag : bool) requires threshold > 0 , forall | i : int , j : int | 0 <= i && i < numbers . len () && 0 <= j && j < numbers . len () ==> numbers [i] - numbers [j] < MAX && - (numbers [i] - numbers [j]) < MAX ensures flag == exists | i : int , j : int | 0 <= i && 0 <= j && i < numbers . len () && j < numbers . len () && i != j && abs_spec (numbers [i] - numbers [j]) < threshold { let mut i = 0 ; while i < numbers . len () invariant 0 <= i <= numbers . len () , forall | x : int , y : int | 0 <= x && x < i && 0 <= y && y < numbers . len () && x != y ==> abs_spec (numbers [x] - numbers [y]) >= threshold decreases numbers . len () - i { let mut j = 0 ; while j < numbers . len () invariant 0 <= i < numbers . len () , 0 <= j <= numbers . len () , forall | x : int , y : int | 0 <= x && x < i && 0 <= y && y < numbers . len () && x != y ==> abs_spec (numbers [x] - numbers [y]) >= threshold , forall | y : int | 0 <= y && y < j && (i as int) != y ==> abs_spec (numbers [i as int] - numbers [y]) >= threshold decreases numbers . len () - j { if i != j { let diff = numbers [i] - numbers [j] ; let abs_diff = if diff < 0 { - diff } else { diff } ; if abs_diff < threshold { return true ; } } j += 1 ; } i += 1 ; } false }

} // verus!