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
pub type CKey = SHTKey ;
# [derive (Eq , PartialEq , Hash)] pub struct SHTKey { pub ukey : u64 , }
pub type AbstractKey = SHTKey ;
pub open spec fn valid_key (key : AbstractKey) -> bool { true }
pub fn is_key_valid (key : & CKey) -> (b : bool) ensures b == valid_key (* key) { true }

} // verus!