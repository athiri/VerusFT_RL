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

pub open spec fn dec_is_none<T>(opt: Option<T>) -> Dec {
    match opt {
        Option::Some(_) => Dec::No,
        Option::None => Dec::Yes,
    }
}


pub proof fn dec_is_none_sound<T>(opt: Option<T>)
    ensures dec_to_bool(dec_is_none(opt)) <==> opt.is_none()
{
}

} // verus!