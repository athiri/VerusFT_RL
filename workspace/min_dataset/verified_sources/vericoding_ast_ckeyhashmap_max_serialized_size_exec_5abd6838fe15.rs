use vstd::prelude::*;
use vstd :: map :: * ;
use vstd :: modes :: * ;
use vstd :: multiset :: * ;
use vstd :: seq :: * ;
use vstd :: set :: * ;
use vstd :: pervasive :: * ;
use vstd :: seq_lib :: * ;
use vstd :: { seq :: * , seq_lib :: * } ;
use vstd :: bytes :: * ;
use vstd :: calc_macro :: * ;
use vstd :: set_lib :: * ;
use vstd :: slice :: * ;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
# [verifier :: opaque] pub open spec fn ckeyhashmap_max_serialized_size () -> usize { 0x100000 }
pub fn ckeyhashmap_max_serialized_size_exec () -> (r : usize) ensures r == ckeyhashmap_max_serialized_size () { reveal (ckeyhashmap_max_serialized_size) ; 0x100000 }

} // verus!