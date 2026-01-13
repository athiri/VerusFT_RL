use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn replace_last_element (first : & Vec < i32 > , second : & Vec < i32 >) -> (replaced_list : Vec < i32 >) requires first . len () > 0 , ensures replaced_list @ == first @ . subrange (0 , first . len () - 1) . add (second @) , { let mut result = Vec :: new () ; for i in 0 .. (first . len () - 1) invariant i <= first . len () - 1 , result @ == first @ . subrange (0 , i as int) , { result . push (first [i]) ; } for i in 0 .. second . len () invariant i <= second . len () , result @ == first @ . subrange (0 , first . len () - 1) . add (second @ . subrange (0 , i as int)) , { result . push (second [i]) ; } result }

} // verus!