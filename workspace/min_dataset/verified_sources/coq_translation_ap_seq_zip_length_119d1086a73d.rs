use vstd::prelude::*;

verus! {

pub open spec fn ap_seq_zip<A, B>(fs: Seq<spec_fn(A) -> B>, xs: Seq<A>) -> Seq<B>
    decreases fs.len()
{
    if fs.len() == 0 || xs.len() == 0 {
        Seq::empty()
    } else {
        seq![fs.first()(xs.first())].add(ap_seq_zip(fs.drop_first(), xs.drop_first()))
    }
}

pub proof fn ap_seq_zip_length<A, B>(fs: Seq<spec_fn(A) -> B>, xs: Seq<A>)
    ensures ap_seq_zip(fs, xs).len() == if fs.len() < xs.len() { fs.len() } else { xs.len() }
    decreases fs.len()
{
    if fs.len() > 0 && xs.len() > 0 {
        ap_seq_zip_length(fs.drop_first(), xs.drop_first());
    }
}

} // verus!
