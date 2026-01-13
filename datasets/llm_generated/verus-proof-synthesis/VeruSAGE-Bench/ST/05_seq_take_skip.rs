use vstd::prelude::*;

fn main() {}

verus! {

proof fn seq_take_skip<A>(s: Seq<A>, n: int)
    requires
        0 <= n <= s.len(),
    ensures
        s.take(n) + s.skip(n) == s,
{
}

}
