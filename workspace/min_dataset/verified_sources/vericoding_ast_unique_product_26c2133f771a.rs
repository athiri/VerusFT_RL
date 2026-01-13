use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn unique_product_precond (arr : & Vec < i32 >) -> bool { true }
fn unique_product (arr : & Vec < i32 >) -> (result : i32) requires unique_product_precond (arr) ensures true { return 0 ; }

} // verus!