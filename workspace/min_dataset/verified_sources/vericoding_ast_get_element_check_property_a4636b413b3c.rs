use vstd::prelude::*;
# [allow (unused_imports)] use vstd :: prelude :: * ;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn f (seq : Seq < u64 > , i : int) -> bool { seq [i] == i + 2 }
fn get_element_check_property (arr : Vec < u64 > , i : usize) -> (ret : u64) requires arr . len () > 0 , 0 < i < arr @ . len () , forall | i : int | f (arr @ , i) , ensures ret == i + 2 , ret == arr @ [i as int] , { assert (f (arr @ , i as int)) ; arr [i] }

} // verus!