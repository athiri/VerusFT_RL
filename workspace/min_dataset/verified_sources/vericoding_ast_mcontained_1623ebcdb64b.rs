use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn strict_sorted (arr : & [i32]) -> bool { forall | k : int , l : int | 0 <= k < l < arr . len () ==> arr [k] < arr [l] }
# [verifier :: loop_isolation (false)] fn mcontained (v : & [i32] , w : & [i32] , n : usize , m : usize) -> (b : bool) requires n <= m && n >= 0 , strict_sorted (v) , strict_sorted (w) , v . len () >= n && w . len () >= m ensures b ==> (forall | k : int | # ! [trigger v [k]] 0 <= k < n ==> (exists | j : int | # ! [trigger w [j]] 0 <= j < m && v [k] == w [j])) { return false ; }

} // verus!