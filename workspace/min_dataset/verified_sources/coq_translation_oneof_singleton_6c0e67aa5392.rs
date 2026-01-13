use vstd::prelude::*;

verus! {

pub open spec fn oneof_outputs<T>(gens: Seq<Set<T>>) -> Set<T> {
    Set::new(|x: T|
        exists|i: int| 0 <= i < gens.len() && gens[i].contains(x)
    )
}


pub proof fn oneof_singleton<T>(gen: Set<T>)
    ensures oneof_outputs(seq![gen]) =~= gen
{
    assert forall|x: T| oneof_outputs(seq![gen]).contains(x) <==> gen.contains(x) by {
        if oneof_outputs(seq![gen]).contains(x) {
            let i = choose|i: int| 0 <= i < 1 && seq![gen][i].contains(x);
            assert(i == 0);
            assert(gen.contains(x));
        }
        if gen.contains(x) {
            assert(seq![gen][0].contains(x));
        }
    }
}

} // verus!