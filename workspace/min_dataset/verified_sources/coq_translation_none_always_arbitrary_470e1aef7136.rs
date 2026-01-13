use vstd::prelude::*;

verus! {

pub open spec fn arbitrary_option<A>(elements: Set<A>) -> Set<Option<A>> {
    Set::new(|o: Option<A>| match o {
        Option::None => true,
        Option::Some(a) => elements.contains(a),
    })
}


pub proof fn none_always_arbitrary<A>(elements: Set<A>)
    ensures arbitrary_option(elements).contains(Option::None)
{
}

} // verus!