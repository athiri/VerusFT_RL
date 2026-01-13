use vstd::prelude::*;

verus! {

pub open spec fn gen_option_bind<T, U>(
    outputs: Set<Option<T>>,
    f: spec_fn(T) -> Set<Option<U>>
) -> Set<Option<U>> {
    Set::new(|o: Option<U>|
        // None if original was None
        (outputs.contains(Option::None) && o.is_none()) ||
        // Or result of applying f to Some value
        exists|x: T| outputs.contains(Option::Some(x)) && f(x).contains(o)
    )
}


pub proof fn gen_option_bind_none<T, U>(outputs: Set<Option<T>>, f: spec_fn(T) -> Set<Option<U>>)
    requires outputs.contains(Option::None)
    ensures gen_option_bind(outputs, f).contains(Option::None)
{
}

} // verus!