use vstd::prelude::*;
use vstd :: assert_by_contradiction ;
use vstd :: map :: * ;
use vstd :: pervasive :: arbitrary ;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
pub open spec (checked) fn aligned (addr : nat , size : nat) -> bool { addr % size == 0 }
pub exec fn aligned_exec (addr : usize , size : usize) -> (res : bool) requires size > 0 ensures res == aligned (addr as nat , size as nat) { addr % size == 0 }

} // verus!