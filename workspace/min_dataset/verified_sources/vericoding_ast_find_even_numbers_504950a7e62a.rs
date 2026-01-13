use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn find_even_numbers_postcond (arr : Seq < i32 > , result : Seq < i32 >) -> bool { forall | j : int | 0 <= j < result . len () ==> { &&& # [trigger] is_even (result [j] as int) &&& exists | k : int | 0 <= k < arr . len () && arr [k] == result [j] } }
spec fn is_even (n : int) -> bool { n % 2 == 0 }
spec fn find_even_numbers_precond (arr : Seq < i32 >) -> bool { true }
fn is_even_exec (n : i32) -> (result : bool) ensures result == is_even (n as int) { n % 2 == 0 }
fn find_even_numbers (arr : Vec < i32 >) -> (result : Vec < i32 >) requires find_even_numbers_precond (arr @) ensures find_even_numbers_postcond (arr @ , result @) { let mut result = Vec :: new () ; let mut i = 0 ; while i < arr . len () invariant 0 <= i <= arr . len () , forall | j : int | 0 <= j < result @ . len () ==> { &&& is_even (result @ [j] as int) &&& exists | k : int | 0 <= k < arr @ . len () && arr @ [k] == result @ [j] } decreases arr . len () - i { if is_even_exec (arr [i]) { result . push (arr [i]) ; } i += 1 ; } result }

} // verus!