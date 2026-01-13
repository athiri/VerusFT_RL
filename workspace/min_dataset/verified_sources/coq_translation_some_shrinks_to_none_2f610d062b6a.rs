use vstd::prelude::*;

verus! {

pub open spec fn shrink_option<A>(o: Option<A>, shrunk_as: Seq<A>) -> Seq<Option<A>>
    where A: std::marker::Copy
    decreases shrunk_as.len()
{
    match o {
        Option::None => seq![],
        Option::Some(a) => {
            seq![Option::None] + shrunk_as.map(|_i: int, x: A| Option::Some(x))
        }
    }
}


pub proof fn some_shrinks_to_none<A>(a: A, shrunk: Seq<A>)
    where A: std::marker::Copy
    ensures shrink_option(Option::Some(a), shrunk)[0] == Option::<A>::None
{
}

} // verus!