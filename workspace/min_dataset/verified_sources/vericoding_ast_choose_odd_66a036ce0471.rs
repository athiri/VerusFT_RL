use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn choose_odd (v : & Vec < u64 >) -> (odd_index : usize) requires exists | q : int | 0 <= q < v . len () && v [q] % 2 == 1 ensures odd_index < v . len () { let mut i = 0 ; while i < v . len () invariant 0 <= i <= v . len () , exists | q : int | i <= q < v . len () && v [q] % 2 == 1 decreases v . len () - i { if v [i] % 2 == 1 { return i ; } i += 1 ; } assert (false) ; 0 }

} // verus!