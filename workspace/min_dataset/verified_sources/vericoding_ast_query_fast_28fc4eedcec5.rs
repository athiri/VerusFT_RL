use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn seq_to_int (a : Seq < i32 >) -> Seq < int > { a . map (| _idx : int , x : i32 | x as int) }
spec fn is_prefix_sum_for (a : Seq < int > , c : Seq < int >) -> bool { &&& a . len () + 1 == c . len () &&& c [0] == 0 &&& forall | i : int | 0 <= i < a . len () ==> c [i + 1] == c [i] + a [i] }
spec fn sum (a : Seq < int > , i : int , j : int) -> int decreases j - i { if i >= j { 0 } else { a [i] + sum (a , i + 1 , j) } }
proof fn aux (a : Seq < int > , c : Seq < int > , i : int , j : int) requires 0 <= i <= j <= a . len () , a . len () + 1 == c . len () , c [0] == 0 , is_prefix_sum_for (a , c) ensures sum (a , i , j) == c [j] - c [i] decreases j - i { if i == j { } else { aux (a , c , i + 1 , j) ; } }
fn query_fast (a : & Vec < i32 > , c : & Vec < i32 > , i : usize , j : usize) -> (r : i32) requires a . len () + 1 == c . len () , c [0] == 0 , i <= j <= a . len () , is_prefix_sum_for (seq_to_int (a @) , seq_to_int (c @)) , forall | k : int | 0 <= k < c @ . len () ==> - 1000000 <= # [trigger] c @ [k] <= 1000000 , ensures r == sum (seq_to_int (a @) , i as int , j as int) { proof { aux (seq_to_int (a @) , seq_to_int (c @) , i as int , j as int) ; } c [j] - c [i] }

} // verus!