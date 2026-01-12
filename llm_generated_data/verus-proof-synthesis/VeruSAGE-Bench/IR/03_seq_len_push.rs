use vstd::prelude::*;
fn main() {}
verus! {

// Pushing to sequence increases length by one
pub proof fn seq_len_push<A>(s: Seq<A>, x: A)
    ensures
        s.push(x).len() == s.len() + 1,
{
}

} // verus!
