use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn search (lst : & Vec < usize > , start_idx : usize , n : usize) -> (result : usize) requires start_idx <= lst . len () , decreases lst . len () - start_idx { return 0 ; }

} // verus!