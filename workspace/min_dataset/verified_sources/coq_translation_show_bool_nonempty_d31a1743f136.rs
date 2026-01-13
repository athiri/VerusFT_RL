use vstd::prelude::*;

verus! {

pub type Str = Seq<nat>;

pub open spec fn show_bool(b: bool) -> Str {
    if b { seq![116nat, 114, 117, 101] }  // "true"
    else { seq![102nat, 97, 108, 115, 101] }  // "false"
}


pub proof fn show_bool_nonempty(b: bool)
    ensures show_bool(b).len() > 0
{
}

} // verus!