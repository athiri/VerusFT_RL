use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn binary_to_decimal_precond (digits : Seq < nat >) -> bool { forall | i : int | 0 <= i < digits . len () ==> (digits [i] == 0 || digits [i] == 1) }
fn binary_to_decimal (digits : Vec < u32 >) -> (result : u32) requires binary_to_decimal_precond (digits @ . map (| i : int , x : u32 | x as nat)) , digits @ . len () == 0 || (digits @ . len () <= 10 && forall | j : int | 0 <= j < digits @ . len () ==> digits [j] <= 1) , { return 0 ; }

} // verus!