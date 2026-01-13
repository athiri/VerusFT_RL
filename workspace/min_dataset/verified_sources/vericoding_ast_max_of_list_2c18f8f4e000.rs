use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn max_of_list_postcond (lst : Seq < u32 > , result : u32) -> bool { (exists | i : int | 0 <= i < lst . len () && lst [i] == result) && (forall | i : int | 0 <= i < lst . len () ==> lst [i] <= result) }
spec fn max_of_list_precond (lst : Seq < u32 >) -> bool { lst . len () > 0 }
fn max_of_list (lst : & Vec < u32 >) -> (result : u32) requires max_of_list_precond (lst @) , ensures max_of_list_postcond (lst @ , result) , { let mut max_val = lst [0] ; let mut i = 1 ; while i < lst . len () invariant 0 < i <= lst . len () , exists | j : int | 0 <= j < i && lst @ [j] == max_val , forall | j : int | 0 <= j < i ==> lst @ [j] <= max_val , decreases lst . len () - i , { if lst [i] > max_val { max_val = lst [i] ; } i += 1 ; } max_val }

} // verus!