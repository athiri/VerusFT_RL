// Variation: WellFormed Trait
// From: source/verismo/src/tspec/wellformed.rs
// Demonstrates: WellFormed trait for type validation

use vstd::prelude::*;

verus! {

pub trait WellFormed {
    spec fn wf(&self) -> bool;
}

impl WellFormed for () {
    open spec fn wf(&self) -> bool {
        true
    }
}

impl WellFormed for u8 {
    open spec fn wf(&self) -> bool {
        true
    }
}

impl WellFormed for u16 {
    open spec fn wf(&self) -> bool {
        true
    }
}

impl WellFormed for u32 {
    open spec fn wf(&self) -> bool {
        true
    }
}

impl WellFormed for u64 {
    open spec fn wf(&self) -> bool {
        true
    }
}

impl WellFormed for usize {
    open spec fn wf(&self) -> bool {
        true
    }
}

impl WellFormed for bool {
    open spec fn wf(&self) -> bool {
        true
    }
}

impl<T1: WellFormed, T2: WellFormed> WellFormed for (T1, T2) {
    open spec fn wf(&self) -> bool {
        self.0.wf() && self.1.wf()
    }
}

impl<T1: WellFormed, T2: WellFormed, T3: WellFormed> WellFormed for (T1, T2, T3) {
    open spec fn wf(&self) -> bool {
        self.0.wf() && self.1.wf() && self.2.wf()
    }
}

impl<T: WellFormed> WellFormed for Option<T> {
    open spec fn wf(&self) -> bool {
        match self {
            Some(v) => v.wf(),
            None => true,
        }
    }
}

impl<T> WellFormed for Ghost<T> {
    open spec fn wf(&self) -> bool {
        true
    }
}

pub struct ValidatedData<T> {
    pub data: T,
    pub valid: bool,
}

impl<T: WellFormed> ValidatedData<T> {
    pub fn new(data: T, valid: bool) -> (result: Self)
        ensures
            result.data == data,
            result.valid == valid,
    {
        ValidatedData { data, valid }
    }

    pub open spec fn wf_data(&self) -> bool {
        self.data.wf()
    }

    pub open spec fn is_valid(&self) -> bool {
        self.valid && self.wf_data()
    }

    pub fn get_data(&self) -> (result: &T)
        ensures
            *result == self.data,
    {
        &self.data
    }

    pub fn is_valid_check(&self) -> (result: bool)
        ensures
            result == self.valid,
    {
        self.valid
    }
}

impl<T: WellFormed> WellFormed for ValidatedData<T> {
    open spec fn wf(&self) -> bool {
        self.is_valid()
    }
}

pub struct DataPair<T1, T2> {
    pub first: T1,
    pub second: T2,
}

impl<T1: WellFormed, T2: WellFormed> DataPair<T1, T2> {
    pub fn new(first: T1, second: T2) -> (result: Self)
        ensures
            result.first == first,
            result.second == second,
    {
        DataPair { first, second }
    }

    pub fn get_first(&self) -> (result: &T1)
        ensures
            *result == self.first,
    {
        &self.first
    }

    pub fn get_second(&self) -> (result: &T2)
        ensures
            *result == self.second,
    {
        &self.second
    }
}

impl<T1: WellFormed, T2: WellFormed> WellFormed for DataPair<T1, T2> {
    open spec fn wf(&self) -> bool {
        self.first.wf() && self.second.wf()
    }
}

pub struct OptionalValue<T> {
    pub value: Option<T>,
}

impl<T: WellFormed> OptionalValue<T> {
    pub fn new(value: Option<T>) -> (result: Self)
        ensures
            match result.value {
                Some(v) => match value {
                    Some(val) => v == val,
                    None => false,
                },
                None => matches!(value, None),
            },
    {
        OptionalValue { value }
    }

    pub fn is_some(&self) -> (result: bool)
        ensures
            result <==> matches!(self.value, Some(_)),
    {
        self.value.is_some()
    }

    pub open spec fn has_value(&self) -> bool {
        matches!(self.value, Some(_))
    }
}

impl<T: WellFormed> WellFormed for OptionalValue<T> {
    open spec fn wf(&self) -> bool {
        self.value.wf()
    }
}

fn test_wellformed_trait() {
    let unit_val = ();
    let u64_val = 42u64;
    let bool_val = true;

    let tuple2 = (10u32, 20u64);
    let tuple3 = (1u8, 2u16, 3u32);

    let opt_some: Option<u64> = Some(100);
    let opt_none: Option<u64> = None;

    let validated = ValidatedData::new(42u64, true);
    let is_valid = validated.is_valid_check();
    let data = validated.get_data();

    let pair = DataPair::new(10u32, 20u64);
    let first = pair.get_first();
    let second = pair.get_second();

    let opt_val = OptionalValue::new(Some(42u64));
    let has_value = opt_val.is_some();

    proof {
        assert(unit_val.wf());
        assert(u64_val.wf());
        assert(bool_val.wf());
        assert(tuple2.wf());
        assert(tuple3.wf());
        assert(opt_some.wf());
        assert(opt_none.wf());
        assert(validated.wf_data());
        assert(is_valid);
        assert(*data == 42);
        assert(pair.wf());
        assert(*first == 10);
        assert(*second == 20);
        assert(opt_val.wf());
        assert(has_value);
    }
}

} // verus!

fn main() {
    test_wellformed_trait();
}
