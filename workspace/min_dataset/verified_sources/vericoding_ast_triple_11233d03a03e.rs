use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn triple_postcond (x : int , result : int) -> bool { result / 3 == x && (result / 3) * 3 == result }
spec fn triple_precond (x : int) -> bool { true }
proof fn mul_div_lemma (x : int) ensures (x * 3) % 3 == 0 , (x * 3) / 3 == x , ((x * 3) / 3) * 3 == x * 3 { }
fn triple (x : i32) -> (result : i32) requires triple_precond (x as int) , - 700000000 <= x <= 700000000 ensures triple_postcond (x as int , result as int) { proof { mul_div_lemma (x as int) ; } x * 3 }

} // verus!