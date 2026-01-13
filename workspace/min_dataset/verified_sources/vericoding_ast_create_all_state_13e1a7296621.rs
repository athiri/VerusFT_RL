use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn create_all_state (increment : i32 , total_number : i32 , level : i32 , current_list : & mut Vec < i32 > , total_list : & mut Vec < Vec < i32 > > ,) requires increment >= 1 , 0 <= level <= total_number , total_number + 1 <= MAX , { }

} // verus!