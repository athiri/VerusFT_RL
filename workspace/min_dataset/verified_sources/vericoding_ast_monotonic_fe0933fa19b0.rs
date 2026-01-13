use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
fn monotonic (l : Vec < i32 >) -> (ret : bool) ensures ret <==> (forall | i : int , j : int | 0 <= i < j < l @ . len () ==> l @ . index (i) <= l @ . index (j)) || (forall | i : int , j : int | 0 <= i < j < l @ . len () ==> l @ . index (i) >= l @ . index (j)) , { if l . len () <= 1 { return true ; } let mut is_non_decreasing = true ; let mut is_non_increasing = true ; for i in 0 .. l . len () - 1 invariant 0 <= i <= l . len () - 1 , is_non_decreasing <==> (forall | k : int , m : int | 0 <= k < m < i + 1 ==> l @ . index (k) <= l @ . index (m)) , is_non_increasing <==> (forall | k : int , m : int | 0 <= k < m < i + 1 ==> l @ . index (k) >= l @ . index (m)) , { if l [i] > l [i + 1] { is_non_decreasing = false ; } if l [i] < l [i + 1] { is_non_increasing = false ; } } is_non_decreasing || is_non_increasing }

} // verus!