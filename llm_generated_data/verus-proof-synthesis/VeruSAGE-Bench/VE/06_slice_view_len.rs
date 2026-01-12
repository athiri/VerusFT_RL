use vstd::prelude::*;

fn main() {}

verus! {

proof fn slice_view_len<'a>(s: &'a [u8])
    ensures
        s@.len() == s.len(),
{
}

}
