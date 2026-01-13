use vstd::prelude::*;

verus! {

pub spec const G_ID: Id = 4;


pub type Id = nat;

pub spec const F_ID: Id = 3;

pub spec const Z_ID: Id = 2;

pub spec const Y_ID: Id = 1;

pub spec const X_ID: Id = 0;


pub proof fn example_predefined_ids_distinct()
    ensures
        X_ID != Y_ID && X_ID != Z_ID && X_ID != F_ID && X_ID != G_ID,
        Y_ID != Z_ID && Y_ID != F_ID && Y_ID != G_ID,
        Z_ID != F_ID && Z_ID != G_ID,
        F_ID != G_ID,
{
}

} // verus!