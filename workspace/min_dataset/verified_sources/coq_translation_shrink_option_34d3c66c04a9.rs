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

} // verus!