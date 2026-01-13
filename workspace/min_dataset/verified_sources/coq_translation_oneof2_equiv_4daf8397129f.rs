use vstd::prelude::*;

verus! {

pub open spec fn oneof_outputs<T>(gens: Seq<Set<T>>) -> Set<T> {
    Set::new(|x: T|
        exists|i: int| 0 <= i < gens.len() && gens[i].contains(x)
    )
}

pub open spec fn oneof2<T>(gen1: Set<T>, gen2: Set<T>) -> Set<T> {
    gen1.union(gen2)
}


pub proof fn oneof2_equiv<T>(gen1: Set<T>, gen2: Set<T>)
    ensures oneof2(gen1, gen2) =~= oneof_outputs(seq![gen1, gen2])
{
    assert forall|x: T| oneof2(gen1, gen2).contains(x) <==>
        oneof_outputs(seq![gen1, gen2]).contains(x) by {
        if oneof2(gen1, gen2).contains(x) {
            if gen1.contains(x) {
                assert(seq![gen1, gen2][0].contains(x));
            } else {
                assert(gen2.contains(x));
                assert(seq![gen1, gen2][1].contains(x));
            }
        }
        if oneof_outputs(seq![gen1, gen2]).contains(x) {
            let i = choose|i: int| 0 <= i < 2 && seq![gen1, gen2][i].contains(x);
            if i == 0 {
                assert(gen1.contains(x));
            } else {
                assert(gen2.contains(x));
            }
        }
    }
}

} // verus!