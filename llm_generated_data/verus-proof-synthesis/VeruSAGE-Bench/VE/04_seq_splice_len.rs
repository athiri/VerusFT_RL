use vstd::prelude::*;

fn main() {}

verus! {

spec fn seq_splice<A>(data: Seq<A>, pos: int, v: Seq<A>) -> Seq<A>
    recommends
        0 <= pos,
        pos + v.len() <= data.len(),
{
    data.take(pos) + v + data.skip(pos + v.len())
}

proof fn seq_splice_len<A>(data: Seq<A>, pos: int, v: Seq<A>)
    requires
        0 <= pos,
        pos + v.len() <= data.len(),
    ensures
        seq_splice(data, pos, v).len() == data.len(),
{
}

}
