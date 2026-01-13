use vstd::prelude::*;

verus! {

pub open spec fn exists_seq_helper<T>(s: Seq<T>, p: spec_fn(T) -> bool, i: int) -> bool
    decreases s.len() - i when i >= 0
{
    if i >= s.len() {
        false
    } else if p(s[i]) {
        true
    } else {
        exists_seq_helper(s, p, i + 1)
    }
}


pub open spec fn bool_to_dec(b: bool) -> Dec {
    if b { Dec::Yes } else { Dec::No }
}

pub open spec fn exists_seq<T>(s: Seq<T>, p: spec_fn(T) -> bool) -> bool {
    exists_seq_helper(s, p, 0)
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

pub open spec fn dec_exists_seq<T>(s: Seq<T>, p: spec_fn(T) -> bool) -> Dec {
    bool_to_dec(exists_seq(s, p))
}


pub proof fn exists_empty_seq<T>(p: spec_fn(T) -> bool)
    ensures !dec_to_bool(dec_exists_seq(Seq::<T>::empty(), p))
{
}

} // verus!