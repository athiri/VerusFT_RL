use vstd::prelude::*;

verus! {

pub enum Dec {
    Yes,
    No,
}

pub open spec fn dec_to_bool(d: Dec) -> bool {
    match d {
        Dec::Yes => true,
        Dec::No => false,
    }
}

pub open spec fn dec_is_some<T>(opt: Option<T>) -> Dec {
    match opt {
        Option::Some(_) => Dec::Yes,
        Option::None => Dec::No,
    }
}


pub proof fn dec_is_some_sound<T>(opt: Option<T>)
    ensures dec_to_bool(dec_is_some(opt)) <==> opt.is_some()
{
}

} // verus!