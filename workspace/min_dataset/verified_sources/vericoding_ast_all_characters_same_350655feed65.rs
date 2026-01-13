use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn all_characters_same_precond (chars : & Vec < char >) -> bool { true }
spec fn all_characters_same_postcond (chars : & Vec < char > , result : bool) -> bool { let char_seq = chars @ ; (result ==> pairwise_equal (char_seq)) && (! result ==> (char_seq . len () != 0 && exists_different_from_first (char_seq))) }
spec fn pairwise_equal < T > (seq : Seq < T >) -> bool { forall | i : int , j : int | 0 <= i < seq . len () && 0 <= j < seq . len () ==> seq [i] == seq [j] }
spec fn exists_different_from_first < T : PartialEq > (seq : Seq < T >) -> bool { seq . len () > 0 && exists | i : int | 1 <= i < seq . len () && # [trigger] seq [i] != seq [0] }
fn all_characters_same (chars : & Vec < char >) -> (result : bool) requires all_characters_same_precond (chars) ensures all_characters_same_postcond (chars , result) { if chars . len () == 0 { return true ; } let first_char = chars [0] ; let mut i = 1 ; while i < chars . len () invariant 1 <= i <= chars . len () , forall | j : int | 0 <= j < i ==> chars @ [j] == first_char , decreases chars . len () - i { if chars [i] != first_char { return false ; } i += 1 ; } proof { assert (forall | j : int , k : int | 0 <= j < chars @ . len () && 0 <= k < chars @ . len () ==> chars @ [j] == chars @ [k]) by { assert (forall | j : int | 0 <= j < chars @ . len () ==> chars @ [j] == first_char) ; } } true }

} // verus!