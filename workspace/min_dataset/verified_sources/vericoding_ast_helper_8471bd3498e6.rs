use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn is_sorted (xs : Seq < int >) -> bool { forall | i : int , j : int | 0 <= i < j < xs . len () ==> xs [i] < xs [j] }
fn helper (ys : & Vec < int > , target : int , idx : usize) -> (result : usize) requires idx <= ys . len () , is_sorted (ys @) , forall | i : int | 0 <= i < idx ==> ys @ [i] < target , ensures idx <= result <= ys . len () , forall | i : int | idx <= i < result ==> ys @ [i] < target , result < ys . len () ==> target <= ys @ [result as int] , decreases ys . len () - idx , { if idx == ys . len () { idx } else if target <= ys [idx] { idx } else { helper (ys , target , idx + 1) } }

} // verus!