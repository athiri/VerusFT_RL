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
pub open spec fn size_to_chunks (sz : int) -> int { (sz + const_persistence_chunk_size () - 1) / const_persistence_chunk_size () }
# [doc = " We model the persistent memory as getting flushed in chunks,"] # [doc = " where each chunk has `const_persistence_chunk_size()` bytes. We refer"] # [doc = " to chunk number `c` as the set of addresses `addr` such that"] # [doc = " `addr / const_persistence_chunk_size() == c`."] pub open spec fn const_persistence_chunk_size () -> int { 8 }
pub exec fn persistence_chunk_size () -> (out : usize) ensures out == const_persistence_chunk_size () { 8 }
pub exec fn size_to_chunks_exec (sz : usize) -> (res : usize) ensures res == size_to_chunks (sz as int) { let whole_chunks = sz / persistence_chunk_size () ; let overflowed_chunk = (sz - whole_chunks * persistence_chunk_size () + persistence_chunk_size () - 1) / persistence_chunk_size () ; whole_chunks + overflowed_chunk }

} // verus!