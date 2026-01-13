use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn secondSmallestAux (s : & Vec < i32 > , i : usize , minIdx : usize , secondIdx : usize) -> (result : i32) requires s . len () > 1 , i <= s . len () , minIdx < s . len () , secondIdx < s . len () , minIdx != secondIdx , minIdx < i , secondIdx < i , ensures exists | j : int | 0 <= j < s . len () && s [j] == result , decreases s . len () - i , { if i == s . len () { return s [secondIdx] ; } let current = s [i] ; let min_val = s [minIdx] ; let second_val = s [secondIdx] ; if current < min_val { secondSmallestAux (s , i + 1 , i , minIdx) } else if current < second_val && current != min_val { secondSmallestAux (s , i + 1 , minIdx , i) } else { secondSmallestAux (s , i + 1 , minIdx , secondIdx) } }

} // verus!