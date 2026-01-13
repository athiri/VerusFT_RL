use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn triple_postcond (x : int , result : int) -> bool { result / 3 == x && (result / 3) * 3 == result }
spec fn triple_precond (x : int) -> bool { true }
proof fn lemma_three_times_div (x : int) ensures (3 * x) / 3 == x , ((3 * x) / 3) * 3 == 3 * x { assert ((3 * x) % 3 == 0) ; lemma_div_mul_cancel (3 * x) ; }
proof fn lemma_div_mul_cancel (n : int) requires n % 3 == 0 ensures n / 3 * 3 == n { }
fn triple (x : i32) -> (result : i32) requires triple_precond (x as int) , - 1000000 <= x <= 1000000 ensures triple_postcond (x as int , result as int) { proof { lemma_three_times_div (x as int) ; } 3 * x }

} // verus!