use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn is_even_spec (x : int) -> bool { x % 2 == 0 }
fn find_even_numbers (arr : & [i32]) -> (even_numbers : Vec < i32 >) requires arr . len () <= MAX , ensures forall | k : int | 0 <= k < even_numbers @ . len () ==> is_even_spec (even_numbers @ [k] as int) , forall | k : int | 0 <= k < even_numbers @ . len () ==> arr @ . contains (even_numbers @ [k]) , forall | x : i32 | even_numbers @ . contains (x) ==> (arr @ . contains (x) && is_even_spec (x as int)) { let mut result = Vec :: new () ; let mut i = 0 ; while i < arr . len () invariant 0 <= i <= arr . len () , forall | k : int | 0 <= k < result @ . len () ==> is_even_spec (result @ [k] as int) , forall | k : int | 0 <= k < result @ . len () ==> arr @ . contains (result @ [k]) , forall | x : i32 | result @ . contains (x) ==> (arr @ . contains (x) && is_even_spec (x as int)) decreases arr . len () - i { let val = arr [i] ; if val % 2 == 0 { result . push (val) ; } i += 1 ; } result }

} // verus!