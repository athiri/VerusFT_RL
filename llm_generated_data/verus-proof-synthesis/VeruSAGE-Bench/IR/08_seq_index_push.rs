use vstd::prelude::*;
fn main() {}
verus! {

// Indexing last element of pushed sequence
pub proof fn seq_index_push<A>(s: Seq<A>, x: A)
    ensures
        s.push(x)[s.len() as int] == x,
{
}

} // verus!
