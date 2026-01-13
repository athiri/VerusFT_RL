use vstd::prelude::*;

verus! {

pub open spec fn gen_option_unwrap_or<T>(
    outputs: Set<Option<T>>,
    default: T
) -> Set<T> {
    Set::new(|x: T|
        outputs.contains(Option::Some(x)) ||
        (outputs.contains(Option::None) && x == default)
    )
}


pub proof fn gen_option_unwrap_some<T>(outputs: Set<Option<T>>, default: T, x: T)
    requires outputs.contains(Option::Some(x))
    ensures gen_option_unwrap_or(outputs, default).contains(x)
{
}

} // verus!