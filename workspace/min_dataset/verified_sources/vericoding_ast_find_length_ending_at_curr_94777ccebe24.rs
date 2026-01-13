use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn find_length_ending_at_curr (prev_nums : & Vec < i32 > , lengths : & Vec < usize > , curr_num : i32) -> (result : usize) requires prev_nums . len () == lengths . len () { return 0 ; }

} // verus!