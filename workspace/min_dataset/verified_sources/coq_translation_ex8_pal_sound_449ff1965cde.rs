use vstd::prelude::*;

verus! {

pub enum PalEv<A> {
    Empty,
    Single(A),
    Step(A, Box<PalEv<A>>),
}

impl<A> PalEv<A> {
    pub open spec fn seq(self) -> Seq<A>
        decreases self
    {
        match self {
            PalEv::Empty => Seq::empty(),
            PalEv::Single(a) => seq![a],
            PalEv::Step(a, mid) => seq![a].add((*mid).seq()).add(seq![a]),
        }
    }
}

pub proof fn lemma_reverse_index<A>(s: Seq<A>, i: int)
    requires 0 <= i < s.len(),
    ensures s.reverse()[i] == s[s.len() - 1 - i]
{
    reveal_with_fuel(Seq::reverse, 1);
    assert(s.reverse()[i] == s[s.len() - 1 - i]);
}


pub proof fn ex8_pal_sound<A>(p: PalEv<A>)
    ensures p.seq() =~= p.seq().reverse()
    decreases p
{
    match p {
        PalEv::Empty => {
            assert(Seq::<A>::empty().reverse() =~= Seq::<A>::empty());
        }
        PalEv::Single(a) => {
            reveal_with_fuel(Seq::reverse, 1);
            assert(seq![a].reverse() =~= seq![a]);
        }
        PalEv::Step(a, mid) => {
            ex8_pal_sound(*mid);
            let s = seq![a].add(mid.seq()).add(seq![a]);
            let rs = s.reverse();

            // Prove s == reverse(s) by extensional equality.
            assert(s.len() == rs.len());
            assert forall|i: int| 0 <= i < s.len() implies s[i] == rs[i] by {
                lemma_reverse_index(s, i);
                assert(rs[i] == s[s.len() - 1 - i]);

                if i == 0 {
                    assert(s[0] == a);
                } else if i == s.len() as int - 1 {
                    assert(s[s.len() as int - 1] == a);
                } else {
                    // Middle region: reduce to mid by symmetry.
                    // s = [a] ++ mid ++ [a]
                    assert(0 < i);
                    assert(i < s.len() as int - 1);

                    let j = i - 1;
                    assert(0 <= j);
                    assert(j < mid.seq().len());

                    // Show s[i] is mid[j]
                    assert(s[i] == mid.seq()[j]);

                    let k = (s.len() as int - 1 - i) - 1;
                    assert(0 <= k);
                    assert(k < mid.seq().len());
                    assert(s[s.len() as int - 1 - i] == mid.seq()[k]);

                    // Use IH: mid == reverse(mid)
                    assert(mid.seq() =~= mid.seq().reverse());
                    assert(mid.seq()[j] == mid.seq().reverse()[j]);
                    lemma_reverse_index(mid.seq(), j);
                    assert(mid.seq().reverse()[j] == mid.seq()[mid.seq().len() - 1 - j]);

                    // Connect: k == len(mid)-1-j
                    assert(k == mid.seq().len() as int - 1 - j);
                    assert(mid.seq()[j] == mid.seq()[k]);
                }
            };
            assert(s =~= rs);
        }
    }
}

} // verus!