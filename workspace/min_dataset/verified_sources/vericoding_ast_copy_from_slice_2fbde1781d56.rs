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
# [verifier :: external_body] pub fn copy_from_slice (bytes : & [u8]) -> (out : Vec < u8 >) ensures out @ == bytes @ { let mut buffer = vec ! [0 ; bytes . len ()] ; let buffer_slice = buffer . as_mut_slice () ; buffer_slice . copy_from_slice (bytes) ; buffer }

} // verus!