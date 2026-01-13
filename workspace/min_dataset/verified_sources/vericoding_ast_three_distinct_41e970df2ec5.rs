use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn three_distinct_spec (s : Seq < char > , i : int) -> (ret : bool) recommends 0 < i && i + 1 < s . len () , { (s [i - 1] != s [i]) && (s [i] != s [i + 1]) && (s [i] != s [i + 1]) }
fn three_distinct (s : & Vec < char > , i : usize) -> (is : bool) requires 0 < i && i + 1 < s . len () , ensures is <==> three_distinct_spec (s @ , i as int) , { s [i - 1] != s [i] && s [i] != s [i + 1] && s [i] != s [i + 1] }

} // verus!