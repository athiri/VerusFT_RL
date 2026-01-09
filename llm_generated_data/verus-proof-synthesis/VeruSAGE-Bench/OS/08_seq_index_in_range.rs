use vstd::prelude::*;

fn main() {}

verus! {

proof fn seq_index_in_range<A>(s: Seq<A>, i: int, j: int, k: int)
    requires
        0 <= i <= j <= s.len(),
        0 <= k < j - i,
    ensures
        s.subrange(i, j)[k] == s[i + k],
{
}

}
