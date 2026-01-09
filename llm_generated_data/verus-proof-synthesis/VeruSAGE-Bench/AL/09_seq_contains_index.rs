use vstd::prelude::*;

fn main() {}

verus! {

proof fn seq_contains_index<A>(s: Seq<A>, x: A)
    requires
        s.contains(x),
    ensures
        exists |i: int| 0 <= i < s.len() && s[i] == x,
{
}

}
