use vstd::prelude::*;

verus! {

pub enum Property {
    Bool { value: bool },
    Conditional { guard: bool, prop: Box<Property> },
    Conjunction { p1: Box<Property>, p2: Box<Property> },
    Disjunction { p1: Box<Property>, p2: Box<Property> },
    ForAll { values: Seq<nat>, prop_fn_result: bool },
}

pub open spec fn eval_property(p: Property) -> bool
    decreases p
{
    match p {
        Property::Bool { value } => value,
        Property::Conditional { guard, prop } => {
            if guard { eval_property(*prop) } else { true }  // Vacuously true
        }
        Property::Conjunction { p1, p2 } => eval_property(*p1) && eval_property(*p2),
        Property::Disjunction { p1, p2 } => eval_property(*p1) || eval_property(*p2),
        Property::ForAll { values, prop_fn_result } => prop_fn_result,
    }
}


pub open spec fn any_pass(props: Seq<Property>) -> bool
    decreases props.len()
{
    if props.len() == 0 {
        false
    } else {
        eval_property(props[0]) || any_pass(props.drop_first())
    }
}

} // verus!