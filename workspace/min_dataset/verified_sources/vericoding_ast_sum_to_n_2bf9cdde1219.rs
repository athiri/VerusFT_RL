use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn spec_sum_to_n (n : nat) -> (ret : nat) decreases n , { if (n == 0) { 0 } else { n + spec_sum_to_n ((n - 1) as nat) } }
fn sum_to_n (n : u32) -> (sum : Option < u32 >) ensures sum . is_some () ==> sum . unwrap () == spec_sum_to_n (n as nat) , { let mut result : u32 = 0 ; let mut i : u32 = 0 ; while i < n invariant i <= n , result == spec_sum_to_n (i as nat) , decreases n - i , { if let Some (new_result) = result . checked_add (i + 1) { result = new_result ; i = i + 1 ; } else { return None ; } } Some (result) }

} // verus!