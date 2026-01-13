use vstd::prelude::*;
use vstd :: arithmetic :: overflow :: CheckedU64 ;
# [cfg (verus_keep_ghost)] use vstd :: arithmetic :: div_mod :: { lemma_fundamental_div_mod , lemma_mod_multiples_vanish } ;
use vstd :: set_lib :: * ;
use vstd :: seq :: * ;
# [cfg (verus_keep_ghost)] use vstd :: arithmetic :: mul :: lemma_mul_inequality ;
use vstd :: seq_lib :: * ;
use vstd :: tokens :: frac :: * ;
use vstd :: bytes :: u64_from_le_bytes ;
use vstd :: slice :: slice_subrange ;
# [cfg (verus_keep_ghost)] use vstd :: std_specs :: hash :: * ;
use vstd :: invariant :: * ;
use vstd :: modes :: * ;
use vstd :: relations :: * ;
use vstd :: bytes ;
use vstd :: layout :: * ;
use vstd :: proph :: * ;
use vstd :: pcm :: * ;
use vstd :: pervasive :: runtime_assert ;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
pub closed spec fn space_needed_for_alignment (addr : int , alignment : int) -> int recommends 0 < alignment { let remainder = addr % alignment ; if remainder == 0 { 0 } else { alignment - remainder } }
pub exec fn get_space_needed_for_alignment (addr : u64 , alignment : u64) -> (result : u64) requires 0 < alignment , ensures result == space_needed_for_alignment (addr as int , alignment as int) { let remainder = addr % alignment ; if remainder == 0 { remainder } else { alignment - remainder } }

} // verus!