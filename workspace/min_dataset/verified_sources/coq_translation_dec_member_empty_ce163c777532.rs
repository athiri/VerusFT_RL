use vstd::prelude::*;

verus! {

pub open spec fn list_contains_helper<T>(s: Seq<T>, x: T, eq: spec_fn(T, T) -> bool, i: int) -> bool
    decreases s.len() - i when i >= 0
{
    if i >= s.len() {
        false
    } else if eq(s[i], x) {
        true
    } else {
        list_contains_helper(s, x, eq, i + 1)
    }
}


pub open spec fn list_contains<T>(s: Seq<T>, x: T, eq: spec_fn(T, T) -> bool) -> bool {
    list_contains_helper(s, x, eq, 0)
}

pub open spec fn bool_to_dec(b: bool) -> Dec {
    if b { Dec::Yes } else { Dec::No }
}


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

pub open spec fn dec_member<T>(s: Seq<T>, x: T, eq: spec_fn(T, T) -> bool) -> Dec {
    bool_to_dec(list_contains(s, x, eq))
}


pub proof fn dec_member_empty<T>(x: T, eq: spec_fn(T, T) -> bool)
    ensures !dec_to_bool(dec_member(Seq::<T>::empty(), x, eq))
{
}

} // verus!