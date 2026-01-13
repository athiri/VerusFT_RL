use vstd::prelude::*;
fn main() {}
verus! {

// Sequence concatenation adds lengths
pub proof fn seq_add_len<A>(s: Seq<A>, t: Seq<A>)
    ensures
        (s + t).len() == s.len() + t.len(),
{
}

} // verus!
