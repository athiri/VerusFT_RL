use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn choose_odd (v : & Vec < u64 >) -> (odd_index : usize) requires exists | q : int | 0 <= q < v . len () && v [q] % 2 == 1 ensures odd_index < v . len () { return 0 ; }

} // verus!