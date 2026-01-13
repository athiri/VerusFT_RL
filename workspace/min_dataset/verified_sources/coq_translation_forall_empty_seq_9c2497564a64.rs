use vstd::prelude::*;

verus! {

pub open spec fn forall_seq_helper<T>(s: Seq<T>, p: spec_fn(T) -> bool, i: int) -> bool
    decreases s.len() - i when i >= 0
{
    if i >= s.len() {
        true
    } else if !p(s[i]) {
        false
    } else {
        forall_seq_helper(s, p, i + 1)
    }
}


pub open spec fn bool_to_dec(b: bool) -> Dec {
    if b { Dec::Yes } else { Dec::No }
}

pub open spec fn forall_seq<T>(s: Seq<T>, p: spec_fn(T) -> bool) -> bool {
    forall_seq_helper(s, p, 0)
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

pub open spec fn dec_forall_seq<T>(s: Seq<T>, p: spec_fn(T) -> bool) -> Dec {
    bool_to_dec(forall_seq(s, p))
}


pub proof fn forall_empty_seq<T>(p: spec_fn(T) -> bool)
    ensures dec_to_bool(dec_forall_seq(Seq::<T>::empty(), p))
{
}

} // verus!