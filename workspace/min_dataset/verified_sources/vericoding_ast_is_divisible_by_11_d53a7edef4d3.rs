use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn is_divisible_by_11_precond (n : int) -> bool { true }
fn is_divisible_by_11 (n : i64) -> (result : bool) requires is_divisible_by_11_precond (n as int) ensures (result ==> exists | k : int | # [trigger] (11 * k) == n as int) && (! result ==> forall | k : int | # [trigger] (11 * k) != n as int) { let result = n % 11 == 0 ; if result { proof { let k = n as int / 11 ; assert (11 * k == n as int) ; } } else { proof { assert (n as int % 11 != 0) ; assert (forall | k : int | # [trigger] (11 * k) != n as int) by { if exists | k : int | # [trigger] (11 * k) == n as int { let k = choose | k : int | # [trigger] (11 * k) == n as int ; assert (n as int % 11 == 0) ; assert (false) ; } } } } result }

} // verus!