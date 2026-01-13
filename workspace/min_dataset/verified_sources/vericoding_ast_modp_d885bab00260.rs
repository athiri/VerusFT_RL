use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn modp_rec (n : nat , p : nat) -> (result : nat) decreases n , { if n == 0 { 1nat % p } else { (modp_rec ((n - 1) as nat , p) * 2) % p } }
fn modmul (a : u32 , b : u32 , p : u32) -> (mul : u32) by (nonlinear_arith) requires p > 0 , ensures mul == ((a as int) * (b as int)) % (p as int) , { let result = ((a as u64) * (b as u64)) % (p as u64) ; result as u32 }
# [verifier :: loop_isolation (false)] fn modp (n : u32 , p : u32) -> (r : u32) by (nonlinear_arith) requires p > 0 , ensures r == modp_rec (n as nat , p as nat) , { let mut result : u32 = 1 % p ; let mut i : u32 = 0 ; while i < n invariant p > 0 , i <= n , result == modp_rec (i as nat , p as nat) , decreases n - i , { result = modmul (result , 2 , p) ; i = i + 1 ; } result }

} // verus!