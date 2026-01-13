use vstd::prelude::*;

verus! {

pub open spec fn list_all_helper<T>(s: Seq<T>, p: spec_fn(T) -> bool, i: int) -> bool
    decreases s.len() - i when i >= 0
{
    if i >= s.len() {
        true
    } else if !p(s[i]) {
        false
    } else {
        list_all_helper(s, p, i + 1)
    }
}


pub open spec fn list_all<T>(s: Seq<T>, p: spec_fn(T) -> bool) -> bool {
    list_all_helper(s, p, 0)
}

pub open spec fn bool_to_dec(b: bool) -> Dec {
    if b { Dec::Yes } else { Dec::No }
}


pub enum Dec {
    Yes,
    No,
}

pub open spec fn dec_all<T>(s: Seq<T>, p: spec_fn(T) -> bool) -> Dec {
    bool_to_dec(list_all(s, p))
}

pub open spec fn dec_to_bool(d: Dec) -> bool {
    match d {
        Dec::Yes => true,
        Dec::No => false,
    }
}


pub proof fn dec_all_empty<T>(p: spec_fn(T) -> bool)
    ensures dec_to_bool(dec_all(Seq::<T>::empty(), p))
{
}

} // verus!