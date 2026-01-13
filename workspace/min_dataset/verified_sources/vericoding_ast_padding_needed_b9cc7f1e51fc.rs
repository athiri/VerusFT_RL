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
# [verifier :: opaque] pub open spec fn spec_padding_needed (offset : nat , align : nat) -> nat { let misalignment = offset % align ; if misalignment > 0 { (align - misalignment) as nat } else { 0 } }
pub const fn padding_needed (offset : usize , align : usize) -> (out : usize) requires align > 0 , ensures out <= align , out as nat == spec_padding_needed (offset as nat , align as nat) { reveal (spec_padding_needed) ; let misalignment = offset % align ; if misalignment > 0 { align - misalignment } else { 0 } }

} // verus!