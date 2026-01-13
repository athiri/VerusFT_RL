use vstd::prelude::*;

verus! {

pub open spec fn arbitrary_option<A>(elements: Set<A>) -> Set<Option<A>> {
    Set::new(|o: Option<A>| match o {
        Option::None => true,
        Option::Some(a) => elements.contains(a),
    })
}


pub proof fn some_from_element<A>(elements: Set<A>, a: A)
    requires elements.contains(a)
    ensures arbitrary_option(elements).contains(Option::Some(a))
{
}

} // verus!