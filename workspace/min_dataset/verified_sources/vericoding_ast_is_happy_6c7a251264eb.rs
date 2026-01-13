use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn three_distinct_spec (s : Seq < char > , i : int) -> (ret : bool) recommends 0 < i && i + 1 < s . len () , { (s [i - 1] != s [i]) && (s [i] != s [i + 1]) && (s [i - 1] != s [i + 1]) }
spec fn happy_spec (s : Seq < char >) -> (ret : bool) { s . len () >= 3 && (forall | i : int | 0 < i && i + 1 < s . len () ==> three_distinct_spec (s , i)) }
fn three_distinct (s : & Vec < char > , i : usize) -> (is : bool) requires 0 < i && i + 1 < s . len () , ensures is <==> three_distinct_spec (s @ , i as int) , { (s [i - 1] != s [i]) && (s [i] != s [i + 1]) && (s [i - 1] != s [i + 1]) }
# [verifier :: loop_isolation (false)] fn is_happy (s : & Vec < char >) -> (happy : bool) ensures happy <==> happy_spec (s @) , { if s . len () < 3 { return false ; } let mut j : usize = 1 ; while j + 1 < s . len () invariant s . len () >= 3 , 1 <= j <= s . len () - 1 , forall | i : int | 1 <= i < j ==> three_distinct_spec (s @ , i) , decreases s . len () - j { if ! three_distinct (s , j) { return false ; } j = j + 1 ; } true }

} // verus!