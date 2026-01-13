use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn find_smallest_postcond (s : Seq < u32 > , result : Option < u32 >) -> bool { match result { None => s . len () == 0 , Some (r) => s . contains (r) && (forall | x : u32 | s . contains (x) ==> r <= x) } }
spec fn find_smallest_precond (s : Seq < u32 >) -> bool { true }
fn find_smallest (s : & Vec < u32 >) -> (result : Option < u32 >) requires find_smallest_precond (s @) , ensures find_smallest_postcond (s @ , result) , { if s . len () == 0 { return None ; } let mut min = s [0] ; let mut i = 1 ; while i < s . len () invariant 0 < s . len () , 1 <= i <= s . len () , s @ . contains (min) , forall | j : int | 0 <= j < i ==> min <= s @ [j] decreases s . len () - i { if s [i] < min { min = s [i] ; } i += 1 ; } Some (min) }

} // verus!