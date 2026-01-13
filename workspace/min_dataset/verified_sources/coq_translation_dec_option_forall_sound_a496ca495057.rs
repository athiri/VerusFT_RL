use vstd::prelude::*;

verus! {

pub open spec fn bool_to_dec(b: bool) -> Dec {
    if b { Dec::Yes } else { Dec::No }
}


pub enum Dec {
    Yes,
    No,
}

pub open spec fn dec_option_forall<T>(opt: Option<T>, p: spec_fn(T) -> bool) -> Dec {
    bool_to_dec(option_forall(opt, p))
}

pub open spec fn dec_to_bool(d: Dec) -> bool {
    match d {
        Dec::Yes => true,
        Dec::No => false,
    }
}

pub open spec fn option_forall<T>(opt: Option<T>, p: spec_fn(T) -> bool) -> bool {
    match opt {
        Option::None => true,
        Option::Some(v) => p(v),
    }
}


pub proof fn dec_option_forall_sound<T>(opt: Option<T>, p: spec_fn(T) -> bool)
    ensures dec_to_bool(dec_option_forall(opt, p)) <==> option_forall(opt, p)
{
}

} // verus!