use vstd::prelude::*;

fn main() {}

verus! {

proof fn seq_index_take<A>(s: Seq<A>, n: int, i: int)
    requires
        0 <= n <= s.len(),
        0 <= i < n,
    ensures
        s.take(n)[i] == s[i],
{
}

}
