use vstd::prelude::*;

fn main() {}

verus! {

proof fn seq_update_index<A>(s: Seq<A>, i: int, v: A)
    requires
        0 <= i < s.len(),
    ensures
        s.update(i, v)[i] == v,
{
}

}
