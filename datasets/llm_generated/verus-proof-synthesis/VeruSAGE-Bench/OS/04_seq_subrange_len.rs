use vstd::prelude::*;

fn main() {}

verus! {

proof fn seq_subrange_len<A>(s: Seq<A>, i: int, j: int)
    requires
        0 <= i <= j <= s.len(),
    ensures
        s.subrange(i, j).len() == j - i,
{
}

}
