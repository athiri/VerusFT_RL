use vstd::prelude::*;
use vstd :: layout ;
use vstd :: raw_ptr :: MemContents ;
use vstd :: simple_pptr :: { self , PPtr } ;
use vstd :: arithmetic :: logarithm :: * ;
use vstd :: arithmetic :: power :: * ;
use vstd :: arithmetic :: power2 :: * ;
use vstd :: bits :: * ;
use vstd :: layout :: is_power_2 ;
use vstd :: relations :: * ;
use vstd :: seq :: * ;
use vstd :: seq_lib :: * ;
use vstd :: atomic :: * ;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
# [verifier :: external_body] # [inline (always)] pub exec fn layout_for_array_is_valid < V : Sized , const N : usize > () ensures layout :: valid_layout (layout :: size_of :: < [V ; N] > () as usize , layout :: align_of :: < [V ; N] > () as usize ,) , layout :: size_of :: < [V ; N] > () as usize as nat == layout :: size_of :: < [V ; N] > () , layout :: align_of :: < [V ; N] > () as usize as nat == layout :: align_of :: < [V ; N] > () , opens_invariants none no_unwind { }

} // verus!