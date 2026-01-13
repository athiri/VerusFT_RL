use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
pub fn havoc_inline_post (v : & mut Vec < u32 > , a : u32 , b : bool) requires forall | k : int | 0 <= k < old (v) . len () ==> old (v) [k] > 0 , a > 0 , b == false , { }

} // verus!