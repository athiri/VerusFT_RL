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
pub open spec fn round_up_to_alignment (addr : int , alignment : int) -> int recommends 0 < alignment { addr + space_needed_for_alignment (addr , alignment) }
pub open spec fn is_aligned (addr : int , alignment : int) -> bool recommends 0 < alignment { addr % alignment == 0 }
pub closed spec fn space_needed_for_alignment (addr : int , alignment : int) -> int recommends 0 < alignment { let remainder = addr % alignment ; if remainder == 0 { 0 } else { alignment - remainder } }
pub proof fn lemma_space_needed_for_alignment_works (addr : int , alignment : int) requires 0 < alignment , ensures 0 <= space_needed_for_alignment (addr , alignment) < alignment , is_aligned (addr + space_needed_for_alignment (addr , alignment) , alignment) { let remainder = addr % alignment ; if remainder != 0 { assert (addr == alignment * (addr / alignment) + (addr % alignment)) by { lemma_fundamental_div_mod (addr , alignment) ; } assert (addr + alignment - remainder == alignment * (addr / alignment) + alignment) ; assert ((addr + alignment - remainder) % alignment == alignment % alignment) by { lemma_mod_multiples_vanish (addr / alignment , alignment , alignment) ; } } }
pub exec fn get_space_needed_for_alignment (addr : u64 , alignment : u64) -> (result : u64) requires 0 < alignment , ensures result == space_needed_for_alignment (addr as int , alignment as int) { let remainder = addr % alignment ; if remainder == 0 { remainder } else { alignment - remainder } }
# [inline] pub exec fn align_checked_u64 (v : & CheckedU64 , alignment : u64) -> (result : CheckedU64) requires 0 < alignment , ensures v @ <= result @ , result @ < v @ + alignment , result @ == round_up_to_alignment (v @ as int , alignment as int) , is_aligned (result @ as int , alignment as int) , { proof { lemma_space_needed_for_alignment_works (v @ as int , alignment as int) ; } if v . is_overflowed () { CheckedU64 :: new_overflowed (Ghost (round_up_to_alignment (v @ as int , alignment as int))) } else { v . add_value (get_space_needed_for_alignment (v . unwrap () , alignment)) } }

} // verus!