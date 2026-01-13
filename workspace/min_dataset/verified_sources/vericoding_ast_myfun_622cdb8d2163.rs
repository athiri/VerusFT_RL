use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
pub fn myfun (a : & mut Vec < i32 > , b : & mut Vec < i32 > , sum : & mut Vec < i32 > , N : i32) requires N > 0 , old (a) . len () == N , old (b) . len () == N , old (sum) . len () == 1 , N < 1000 , ensures sum [0] <= 2 * N , { let mut total : i32 = 0 ; let mut i : usize = 0 ; while i < N as usize invariant i <= N as usize , total <= 2 * (i as i32) , a . len () == N , b . len () == N , sum . len () == 1 , N > 0 , N < 1000 , decreases N as usize - i { if i < a . len () && i < b . len () { let val_a = if a [i] > 1 { 1 } else if a [i] < 0 { 0 } else { a [i] } ; let val_b = if b [i] > 1 { 1 } else if b [i] < 0 { 0 } else { b [i] } ; total = total + val_a + val_b ; } i = i + 1 ; } sum . set (0 , total) ; }

} // verus!