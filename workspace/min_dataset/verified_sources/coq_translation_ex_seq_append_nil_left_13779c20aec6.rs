use vstd::prelude::*;
use vstd::seq_lib::*;

verus! {

pub type ListN = Seq<nat>;


pub proof fn ex_seq_append_nil_left(xs: ListN)
    ensures seq![].add(xs) =~= xs
{
    assert_seqs_equal!(seq![].add(xs), xs);
}

} // verus!