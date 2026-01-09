use vstd::prelude::*;

fn main() {}

verus! {

proof fn seq_index_skip<A>(s: Seq<A>, n: int, i: int)
    requires
        0 <= n <= s.len(),
        0 <= i < s.len() - n,
    ensures
        s.skip(n)[i] == s[n + i],
{
}

}
