use vstd::prelude::*;

verus! {

pub open spec fn bool_to_dec(b: bool) -> Dec {
    if b { Dec::Yes } else { Dec::No }
}

pub open spec fn list_any_helper<T>(s: Seq<T>, p: spec_fn(T) -> bool, i: int) -> bool
    decreases s.len() - i when i >= 0
{
    if i >= s.len() {
        false
    } else if p(s[i]) {
        true
    } else {
        list_any_helper(s, p, i + 1)
    }
}


pub enum Dec {
    Yes,
    No,
}

pub open spec fn dec_any<T>(s: Seq<T>, p: spec_fn(T) -> bool) -> Dec {
    bool_to_dec(list_any(s, p))
}

pub open spec fn dec_to_bool(d: Dec) -> bool {
    match d {
        Dec::Yes => true,
        Dec::No => false,
    }
}

pub open spec fn list_any<T>(s: Seq<T>, p: spec_fn(T) -> bool) -> bool {
    list_any_helper(s, p, 0)
}


pub proof fn dec_any_sound<T>(s: Seq<T>, p: spec_fn(T) -> bool)
    ensures dec_to_bool(dec_any(s, p)) == list_any(s, p)
{
}

} // verus!