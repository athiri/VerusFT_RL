use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn prime (n : nat) -> bool { n > 1 && (forall | nr : nat | 1 < nr < n ==> # [trigger] (n % nr) != 0) }
fn test_primeness (n : u64) -> (result : bool) ensures result == prime (n as nat) { if n <= 1 { assert (! prime (n as nat)) ; return false ; } let mut i = 2u64 ; while i < n invariant 2 <= i <= n , n > 1 , forall | nr : nat | 2 <= nr < i ==> # [trigger] ((n as nat) % nr) != 0 , decreases n - i { if n % i == 0 { assert (1 < (i as nat) < (n as nat)) ; assert ((n as nat) % (i as nat) == 0) ; assert (! prime (n as nat)) ; return false ; } i = i + 1 ; } assert (i == n) ; assert (forall | nr : nat | 2 <= nr < (n as nat) ==> # [trigger] ((n as nat) % nr) != 0) ; assert (forall | nr : nat | 1 < nr < (n as nat) ==> # [trigger] ((n as nat) % nr) != 0) ; assert (prime (n as nat)) ; return true ; }

} // verus!