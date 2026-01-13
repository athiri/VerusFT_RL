use vstd::prelude::*;

verus! {

pub open spec fn snd<A, B>(p: (A, B)) -> B { p.1 }


pub proof fn snd_pair<A, B>(a: A, b: B) ensures snd((a, b)) == b {}

} // verus!