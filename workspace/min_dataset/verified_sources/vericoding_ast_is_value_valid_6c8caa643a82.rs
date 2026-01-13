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
pub type AbstractValue = Seq < u8 > ;
pub open spec fn valid_value (value : AbstractValue) -> bool { value . len () < max_val_len () }
pub open spec fn max_val_len () -> int { 1024 }
pub fn is_value_valid (val : & Vec < u8 >) -> (b : bool) ensures b == valid_value (val @) { val . len () < 1024 }

} // verus!