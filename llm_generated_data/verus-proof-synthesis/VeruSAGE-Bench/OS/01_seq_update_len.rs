use vstd::prelude::*;

fn main() {}

verus! {

proof fn seq_update_len<A>(s: Seq<A>, i: int, v: A)
    requires
        0 <= i < s.len(),
    ensures
        s.update(i, v).len() == s.len(),
{
}

}
