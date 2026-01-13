use vstd::prelude::*;

verus! {

pub proof fn gen_some_no_none<T>(inner_outputs: Set<T>)
    ensures !gen_some_outputs(inner_outputs).contains(Option::None)
{
}


pub open spec fn gen_option_or_else<T>(
    outputs: Set<Option<T>>,
    alt: Set<Option<T>>
) -> Set<Option<T>> {
    Set::new(|o: Option<T>|
        // Some values from original
        (o.is_some() && outputs.contains(o)) ||
        // Or if original can be None, include alternative
        (outputs.contains(Option::None) && alt.contains(o))
    )
}

pub open spec fn gen_some_outputs<T>(inner_outputs: Set<T>) -> Set<Option<T>> {
    Set::new(|o: Option<T>| match o {
        Option::None => false,
        Option::Some(x) => inner_outputs.contains(x),
    })
}


pub proof fn gen_option_or_else_some<T>(inner: Set<T>, alt: Set<Option<T>>)
    ensures gen_option_or_else(gen_some_outputs(inner), alt) =~= gen_some_outputs(inner)
{
    let outputs = gen_some_outputs(inner);
    assert forall|o: Option<T>| gen_option_or_else(outputs, alt).contains(o) <==>
        outputs.contains(o) by {
        gen_some_no_none(inner);
    }
}

} // verus!