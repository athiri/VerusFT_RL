use vstd::prelude::*;
use vstd :: pervasive :: runtime_assert ;
use vstd :: bytes :: * ;
use vstd :: arithmetic :: div_mod :: * ;
use vstd :: slice :: * ;
use vstd :: bytes ;
use vstd :: layout :: * ;
use vstd :: invariant :: * ;
use vstd :: seq :: * ;
use vstd :: seq_lib :: * ;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
# [verifier :: external_body] pub fn copy_from_slice (bytes : & [u8]) -> (out : Vec < u8 >) ensures out @ == bytes @ { let mut buffer = vec ! [0 ; bytes . len ()] ; let buffer_slice = buffer . as_mut_slice () ; buffer_slice . copy_from_slice (bytes) ; buffer }

} // verus!