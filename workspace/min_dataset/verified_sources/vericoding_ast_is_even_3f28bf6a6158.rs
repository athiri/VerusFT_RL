use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn even (n : int) -> bool recommends n >= 0 decreases n { if n == 0 { true } else if n > 0 { ! even (n - 1) } else { arbitrary () } }
proof fn lemma_even_mod_equiv (n : nat) ensures even (n as int) <==> (n % 2 == 0) decreases n { if n == 0 { assert (even (0int) == true) ; assert (0nat % 2 == 0) ; } else if n == 1 { assert (even (1int) == ! even (0int)) ; assert (! even (0int) == false) ; assert (1nat % 2 == 1) ; } else { lemma_even_mod_equiv ((n - 1) as nat) ; assert (even (n as int) == ! even ((n - 1) as int)) ; assert (((n - 1) % 2 == 0) ==> (n % 2 == 1)) ; assert (((n - 1) % 2 == 1) ==> (n % 2 == 0)) ; } }
fn is_even (n : u32) -> (r : bool) requires n >= 0 , ensures r <==> even (n as int) { proof { lemma_even_mod_equiv (n as nat) ; } n % 2 == 0 }

} // verus!