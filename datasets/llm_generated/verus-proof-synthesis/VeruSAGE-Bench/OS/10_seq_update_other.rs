use vstd::prelude::*;

fn main() {}

verus! {

proof fn seq_update_other<A>(s: Seq<A>, i: int, j: int, v: A)
    requires
        0 <= i < s.len(),
        0 <= j < s.len(),
        i != j,
    ensures
        s.update(i, v)[j] == s[j],
{
}

}
