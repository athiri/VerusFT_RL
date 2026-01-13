use vstd::prelude::*;

verus! {

pub open spec fn ap_seq_zip<A, B>(
    fs: Seq<spec_fn(A) -> B>,
    xs: Seq<A>
) -> Seq<B> {
    let min_len = if fs.len() < xs.len() { fs.len() } else { xs.len() };
    Seq::new(min_len, |i: int| fs[i](xs[i]))
}

pub open spec fn pure_seq<A>(a: A, len: nat) -> Seq<A> {
    Seq::new(len, |_i: int| a)
}


pub proof fn ap_seq_zip_identity<A>(xs: Seq<A>)
    ensures ap_seq_zip(pure_seq(|a: A| a, xs.len()), xs) =~= xs
{
    let fs = pure_seq(|a: A| a, xs.len());
    let result = ap_seq_zip(fs, xs);
    assert(result.len() == xs.len());
    assert forall|i: int| 0 <= i < xs.len() as int implies result[i] == xs[i] by {
        assert(fs[i] == (|a: A| a));
        assert(result[i] == fs[i](xs[i]));
        assert(result[i] == xs[i]);
    };
}

} // verus!