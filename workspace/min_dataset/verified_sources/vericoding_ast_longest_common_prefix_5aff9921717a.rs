use vstd::prelude::*;

verus! {

pub spec const MAX: int = i32::MAX as int;
pub spec const MIN: int = i32::MIN as int;
spec fn longest_common_prefix_precond (str1 : Seq < char > , str2 : Seq < char >) -> bool { true }
spec fn longest_common_prefix_postcond (str1 : Seq < char > , str2 : Seq < char > , result : Seq < char >) -> bool { &&& result . len () <= str1 . len () &&& result == str1 . subrange (0 , result . len () as int) &&& result . len () <= str2 . len () &&& result == str2 . subrange (0 , result . len () as int) &&& (result . len () == str1 . len () || result . len () == str2 . len () || (result . len () < str1 . len () && result . len () < str2 . len () && str1 [result . len () as int] != str2 [result . len () as int])) }
fn longest_common_prefix (str1 : & Vec < char > , str2 : & Vec < char >) -> (result : Vec < char >) requires longest_common_prefix_precond (str1 @ , str2 @) ensures longest_common_prefix_postcond (str1 @ , str2 @ , result @) { let mut result = Vec :: new () ; let mut i = 0 ; while i < str1 . len () && i < str2 . len () && str1 [i] == str2 [i] invariant 0 <= i <= str1 . len () , 0 <= i <= str2 . len () , result @ == str1 @ . subrange (0 , i as int) , result @ == str2 @ . subrange (0 , i as int) , forall | j : int | 0 <= j < i ==> str1 @ [j] == str2 @ [j] decreases str1 @ . len () - i { result . push (str1 [i]) ; i += 1 ; } result }

} // verus!