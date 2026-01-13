use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
# [doc = "\n      Ather, Mohammad Faiz (s4648481/3)\n      CSSE3100\n      Assignment 3\n      The University of Queensland\n     "] fn tangent (r : Vec < i32 > , x : Vec < i32 >) -> (found : bool) requires true , ensures true , { r . len () == x . len () }

} // verus!