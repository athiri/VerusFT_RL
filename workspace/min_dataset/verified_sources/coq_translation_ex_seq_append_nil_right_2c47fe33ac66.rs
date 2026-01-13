use vstd::prelude::*;
use vstd::seq_lib::*;

verus! {

pub type ListN = Seq<nat>;


pub proof fn ex_seq_append_nil_right(xs: ListN)
    ensures xs.add(seq![]) =~= xs
{
    assert_seqs_equal!(xs.add(seq![]), xs);
}

} // verus!