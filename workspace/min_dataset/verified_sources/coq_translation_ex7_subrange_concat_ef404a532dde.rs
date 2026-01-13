use vstd::prelude::*;
use vstd::seq_lib::*;

verus! {

pub proof fn ex7_subrange_concat<A>(s: Seq<A>, i: int)
    requires 0 <= i <= s.len(),
    ensures s =~= s.subrange(0, i).add(s.subrange(i, s.len() as int))
{
    let t1 = s.subrange(0, i);
    let t2 = s.subrange(i, s.len() as int);
    let t = t1.add(t2);

    assert_seqs_equal!(s == t);
    assert(s =~= t);
}

} // verus!