use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn derivative (xs : & Vec < usize >) -> (ret : Option < Vec < usize > >) ensures ret . is_some () ==> xs @ . len () == 0 || xs @ . map (| i : int , x | i * x) . skip (1) =~= ret . unwrap () @ . map_values (| x | x as int) , { return None ; }

} // verus!