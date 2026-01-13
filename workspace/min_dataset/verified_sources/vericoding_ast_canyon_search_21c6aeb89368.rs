use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn canyon_search_precond (a : Seq < i32 > , b : Seq < i32 >) -> bool { a . len () > 0 && b . len () > 0 && is_sorted (a) && is_sorted (b) }
spec fn spec_nat_abs_diff (x : i32 , y : i32) -> nat { let diff = (x as int) - (y as int) ; if diff >= 0 { diff as nat } else { (- diff) as nat } }
spec fn is_sorted (s : Seq < i32 >) -> bool { forall | i : int , j : int | 0 <= i < j < s . len () ==> s [i] <= s [j] }
fn nat_abs_diff (x : i32 , y : i32) -> (result : usize) ensures result as nat == spec_nat_abs_diff (x , y) { let diff = x as i64 - y as i64 ; if diff >= 0 { diff as usize } else { (- diff) as usize } }
fn canyon_search_aux (a : & Vec < i32 > , b : & Vec < i32 > , m : usize , n : usize , d : usize) -> (result : usize) requires a . len () > 0 , b . len () > 0 , m <= a . len () , n <= b . len () , ensures result <= d , decreases a . len () + b . len () - m - n { if m >= a . len () || n >= b . len () { return d ; } let current_diff = nat_abs_diff (a [m] , b [n]) ; let min_so_far = if current_diff < d { current_diff } else { d } ; if a [m] <= b [n] { canyon_search_aux (a , b , m + 1 , n , min_so_far) } else { canyon_search_aux (a , b , m , n + 1 , min_so_far) } }
fn canyon_search (a : & Vec < i32 > , b : & Vec < i32 >) -> (result : usize) requires canyon_search_precond (a @ , b @) , { let initial_diff = nat_abs_diff (a [0] , b [0]) ; canyon_search_aux (a , b , 0 , 0 , initial_diff) }

} // verus!