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
pub type Arg = Vec < u8 > ;
pub open spec fn optional_value_view (ov : Option :: < Vec :: < u8 > >) -> Option :: < Seq :: < u8 > > { match ov { Some (v) => Some (v @) , None => None , } }
# [doc = " Clone a Vec<u8>."] # [doc = ""] # [doc = " Implemented as a loop, so might not be as efficient as the"] # [doc = " `Vec::clone` method."] pub fn clone_vec_u8 (v : & Vec < u8 >) -> (out : Vec < u8 >) ensures out @ == v @ { let mut out : Arg = Vec :: with_capacity (v . len ()) ; let mut i = 0 ; while i < v . len () invariant i <= v . len () , i == out . len () , forall | j | # ! [auto] 0 <= j < i ==> out @ [j] == v @ [j] , decreases v . len () - i , { out . push (v [i]) ; i = i + 1 ; } proof { assert_seqs_equal ! (out @, v @) ; } out }
pub fn clone_optional_value (ov : & Option :: < Vec :: < u8 > >) -> (res : Option :: < Vec :: < u8 > >) ensures optional_value_view (* ov) == optional_value_view (res) { match ov . as_ref () { Some (v) => Some (clone_vec_u8 (v)) , None => None , } }

} // verus!