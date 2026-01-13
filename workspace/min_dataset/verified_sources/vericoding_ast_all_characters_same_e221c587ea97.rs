use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn all_characters_same (char_arr : & [u8]) -> (result : bool) ensures result == (forall | i : int | 1 <= i < char_arr @ . len () ==> char_arr [0] == # [trigger] char_arr [i]) , { if char_arr . len () == 0 { return true ; } let first_char = char_arr [0] ; for i in 1 .. char_arr . len () invariant first_char == char_arr [0] , forall | j : int | 1 <= j < i ==> first_char == # [trigger] char_arr [j] , { if char_arr [i] != first_char { assert (first_char != char_arr [i as int]) ; assert (char_arr [0] != char_arr [i as int]) ; assert (1 <= i < char_arr @ . len ()) ; assert (! (forall | k : int | 1 <= k < char_arr @ . len () ==> char_arr [0] == # [trigger] char_arr [k])) ; return false ; } assert (first_char == char_arr [i as int]) ; } assert (forall | j : int | 1 <= j < char_arr @ . len () ==> char_arr [0] == # [trigger] char_arr [j]) ; true }

} // verus!