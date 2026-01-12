use vstd::prelude::*;

fn main() {}

verus! {

proof fn seq_push_last<A>(s: Seq<A>, x: A)
    ensures
        s.push(x).last() == x,
{
}

}
