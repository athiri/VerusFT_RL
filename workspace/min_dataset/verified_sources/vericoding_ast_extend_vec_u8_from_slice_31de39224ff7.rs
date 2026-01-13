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
# [inline] pub exec fn extend_vec_u8_from_slice (v : & mut Vec < u8 > , s : & [u8]) ensures v @ == old (v) @ + s @ , { v . extend_from_slice (s) ; assert (v @ =~= old (v) @ + s @) ; }

} // verus!