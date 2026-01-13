use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn has_chord_intersection (n : usize , chords : Vec < Vec < usize > >) -> (result : bool) requires n >= 2 , forall | i : int | 0 <= i < chords . len () ==> (# [trigger] chords [i] . len () == 2 && chords [i] [0] >= 1 && chords [i] [0] <= 2 * n && chords [i] [1] >= 1 && chords [i] [1] <= 2 * n) { return false ; }

} // verus!