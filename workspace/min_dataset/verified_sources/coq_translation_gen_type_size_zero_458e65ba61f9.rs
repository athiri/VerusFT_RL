use vstd::prelude::*;

verus! {

pub open spec fn gen_prod_types(components: Set<Ty>) -> Set<Ty> {
    Set::new(|t: Ty|
        exists|t1: Ty, t2: Ty|
            components.contains(t1) && components.contains(t2) &&
            t == Ty::TProd { t1: Box::new(t1), t2: Box::new(t2) }
    )
}

pub open spec fn base_types() -> Set<Ty> {
    Set::new(|t: Ty| t == Ty::TBool || t == Ty::TNat || t == Ty::TUnit)
}

pub open spec fn gen_arrow_types(components: Set<Ty>) -> Set<Ty> {
    Set::new(|t: Ty|
        exists|t1: Ty, t2: Ty|
            components.contains(t1) && components.contains(t2) &&
            t == Ty::TArrow { t1: Box::new(t1), t2: Box::new(t2) }
    )
}

pub open spec fn gen_sum_types(components: Set<Ty>) -> Set<Ty> {
    Set::new(|t: Ty|
        exists|t1: Ty, t2: Ty|
            components.contains(t1) && components.contains(t2) &&
            t == Ty::TSum { t1: Box::new(t1), t2: Box::new(t2) }
    )
}


pub enum Ty {
    TBool,
    TNat,
    TUnit,
    TProd { t1: Box<Ty>, t2: Box<Ty> },  // Product type
    TSum { t1: Box<Ty>, t2: Box<Ty> },   // Sum type
    TArrow { t1: Box<Ty>, t2: Box<Ty> }, // Function type
}

pub open spec fn gen_type_sized(size: nat) -> Set<Ty>
    decreases size
{
    if size == 0 {
        Set::empty()
    } else if size == 1 {
        base_types()
    } else {
        // Include base types plus compound types built from smaller types
        let smaller = gen_type_sized((size - 1) as nat);
        let bases = base_types();
        let prods = gen_prod_types(smaller);
        let sums = gen_sum_types(smaller);
        let arrows = gen_arrow_types(smaller);
        bases.union(prods).union(sums).union(arrows)
    }
}


pub proof fn gen_type_size_zero()
    ensures gen_type_sized(0) == Set::<Ty>::empty()
{
}

} // verus!