use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn comb (n : nat , k : nat) -> nat recommends 0 <= k <= n decreases n { if k == 0 || k == n { 1 } else if k > n { 0 } else { comb (sub (n , 1) , k) + comb (sub (n , 1) , sub (k , 1)) } }
# [verifier :: external_body] fn comb_method (n : u64 , k : u64) -> (result : u64) requires 0 <= k <= n , ensures result as nat == comb (n as nat , k as nat) , { if k == 0 || k == n { return 1 ; } let actual_k = if k > n - k { n - k } else { k } ; let mut dp = Vec :: new () ; dp . push (1u64) ; for i in 1 ..= n { let mut new_row = Vec :: new () ; new_row . push (1u64) ; for j in 1 ..= actual_k { if j > i { new_row . push (0u64) ; } else if j == i { new_row . push (1u64) ; } else { let prev_val = if j < dp . len () as u64 { dp [j as usize] } else { 0u64 } ; let prev_val_minus_1 = if j > 0 && (j - 1) < dp . len () as u64 { dp [(j - 1) as usize] } else { 0u64 } ; new_row . push (prev_val + prev_val_minus_1) ; } } dp = new_row ; } if actual_k < dp . len () as u64 { dp [actual_k as usize] } else { 0 } }

} // verus!