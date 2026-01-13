use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn comb (n : nat , k : nat) -> nat recommends 0 <= k <= n decreases n { if k == 0 || k == n { 1 } else if k > n { 0 } else { comb (sub (n , 1) , k) + comb (sub (n , 1) , sub (k , 1)) } }
# [verifier :: external_body] fn comb_method (n : u64 , k : u64) -> (result : u64) requires 0 <= k <= n , ensures result as nat == comb (n as nat , k as nat) , { return 0 ; }

} // verus!