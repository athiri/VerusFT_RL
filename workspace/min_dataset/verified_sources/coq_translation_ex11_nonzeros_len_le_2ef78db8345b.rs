use vstd::prelude::*;

verus! {

pub type NatList = Seq<nat>;

pub open spec fn nonzeros(xs: NatList) -> NatList
    decreases xs.len()
{
    if xs.len() == 0 {
        Seq::empty()
    } else {
        let x = xs[0];
        let rest = nonzeros(xs.skip(1));
        if x == 0 {
            rest
        } else {
            seq![x].add(rest)
        }
    }
}


pub proof fn ex11_nonzeros_len_le(xs: NatList)
    ensures nonzeros(xs).len() <= xs.len()
    decreases xs.len()
{
    if xs.len() == 0 {
        assert(nonzeros(xs).len() == 0);
    } else {
        let tail = xs.skip(1);
        ex11_nonzeros_len_le(tail);

        let x = xs[0];
        if x == 0 {
            assert(nonzeros(xs) == nonzeros(tail));
            assert(nonzeros(xs).len() <= tail.len());
            assert(tail.len() < xs.len());
        } else {
            assert(nonzeros(xs) == seq![x].add(nonzeros(tail)));
            assert(nonzeros(xs).len() == nonzeros(tail).len() + 1);
            assert(nonzeros(tail).len() <= tail.len());
            assert(nonzeros(xs).len() <= tail.len() + 1);
            assert(tail.len() + 1 == xs.len());
        }
    }
}

} // verus!