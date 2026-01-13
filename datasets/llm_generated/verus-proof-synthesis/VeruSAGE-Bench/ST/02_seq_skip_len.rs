use vstd::prelude::*;

fn main() {}

verus! {

proof fn seq_skip_len<A>(s: Seq<A>, n: int)
    requires
        0 <= n <= s.len(),
    ensures
        s.skip(n).len() == s.len() - n,
{
}

}
