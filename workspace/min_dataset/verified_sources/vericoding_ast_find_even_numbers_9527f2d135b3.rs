use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn is_even_spec (x : int) -> bool { x % 2 == 0 }
fn find_even_numbers (arr : & [i32]) -> (even_numbers : Vec < i32 >) requires arr . len () <= MAX , ensures forall | k : int | 0 <= k < even_numbers @ . len () ==> is_even_spec (even_numbers @ [k] as int) , forall | k : int | 0 <= k < even_numbers @ . len () ==> arr @ . contains (even_numbers @ [k]) , forall | x : i32 | even_numbers @ . contains (x) ==> (arr @ . contains (x) && is_even_spec (x as int)) { let mut result = Vec :: new () ; for i in 0 .. arr . len () invariant forall | k : int | 0 <= k < result @ . len () ==> is_even_spec (result @ [k] as int) , forall | k : int | 0 <= k < result @ . len () ==> exists | j : int | 0 <= j < i && arr @ [j] == # [trigger] result @ [k] , forall | x : i32 | result @ . contains (x) ==> (exists | j : int | 0 <= j < i && arr @ [j] == x) && is_even_spec (x as int) { if arr [i] % 2 == 0 { result . push (arr [i]) ; } } result }

} // verus!