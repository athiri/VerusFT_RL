use vstd::prelude::*;

verus! {

pub enum Ty {
    TBool,                                    // Boolean type
    TNat,                                     // Natural number type
    TArrow { t1: Box<Ty>, t2: Box<Ty> },     // Function type: t1 -> t2
}


pub open spec fn well_formed(ty: Ty) -> bool {
    true  // Our type syntax ensures well-formedness
}

} // verus!