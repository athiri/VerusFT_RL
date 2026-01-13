use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn abs (i : i32) -> (res : i32) requires i != MIN , ensures i < 0 ==> res == - i , i >= 0 ==> res == i { if i < 0 { - i } else { i } }

} // verus!