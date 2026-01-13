use vstd::prelude::*;

verus! {

pub open spec fn gen_option_zip<T, U>(
    out1: Set<Option<T>>,
    out2: Set<Option<U>>
) -> Set<Option<(T, U)>> {
    Set::new(|o: Option<(T, U)>| match o {
        Option::None =>
            out1.contains(Option::None) || out2.contains(Option::None),
        Option::Some((x, y)) =>
            out1.contains(Option::Some(x)) && out2.contains(Option::Some(y)),
    })
}


pub proof fn gen_option_zip_none<T, U>(out1: Set<Option<T>>, out2: Set<Option<U>>)
    requires out1.contains(Option::None) || out2.contains(Option::None)
    ensures gen_option_zip(out1, out2).contains(Option::None)
{
}

} // verus!