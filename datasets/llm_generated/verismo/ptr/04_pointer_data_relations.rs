// Variation: Pointer Data Relations
// From: source/verismo/src/ptr/ptr_s.rs (sw_eq, only_val_updated, spec_read_rel, spec_write_rel)
// Demonstrates: Relationships between pointer states and values

use vstd::prelude::*;

verus! {

pub struct PointerData<V> {
    pub addr: usize,
    pub value: Option<V>,
    pub flags: u32,
}

impl<V> PointerData<V> {
    pub fn new(addr: usize, value: Option<V>, flags: u32) -> (result: Self)
        ensures
            result.addr == addr,
            result.value == value,
            result.flags == flags,
    {
        PointerData { addr, value, flags }
    }

    pub open spec fn id(&self) -> int {
        self.addr as int
    }

    pub open spec fn is_assigned(&self) -> bool {
        matches!(self.value, Some(_))
    }

    pub fn check_assigned(&self) -> (result: bool)
        ensures
            result <==> self.is_assigned(),
    {
        self.value.is_some()
    }

    pub open spec fn only_val_updated(&self, old: &Self) -> bool {
        &&& self.addr == old.addr
        &&& self.flags == old.flags
        &&& matches!(self.value, Some(_))
    }

    pub fn check_only_val_updated(&self, old: &Self) -> (result: bool)
        ensures
            result ==> self.only_val_updated(old),
    {
        self.addr == old.addr && self.flags == old.flags && self.value.is_some()
    }
}

impl<V: PartialEq> PointerData<V> {
    pub open spec fn sw_eq(&self, other: &Self) -> bool {
        &&& self.addr == other.addr
        &&& self.value == other.value
        &&& self.flags == other.flags
    }

    pub fn check_sw_eq(&self, other: &Self) -> (result: bool)
    {
        self.addr == other.addr && self.value == other.value && self.flags == other.flags
    }

    pub open spec fn only_flags_updated(&self, old: &Self) -> bool {
        &&& self.addr == old.addr
        &&& self.value == old.value
    }

    pub fn check_only_flags_updated(&self, old: &Self) -> (result: bool)
    {
        self.addr == old.addr && self.value == old.value
    }

    pub open spec fn spec_write_rel(&self, prev: &Self, new_val: Option<V>) -> bool {
        &&& self.addr == prev.addr
        &&& self.flags == prev.flags
        &&& self.value == new_val
    }

    pub open spec fn spec_read_rel(&self, val: V) -> bool {
        matches!(self.value, Some(_)) ==> self.value == Some(val)
    }

    pub fn check_spec_read_rel(&self, val: V) -> (result: bool)
    {
        match &self.value {
            Some(v) => *v == val,
            None => true,
        }
    }
}

pub struct PointerState<V> {
    pub current: PointerData<V>,
    pub generation: u64,
}

impl<V> PointerState<V> {
    pub fn new(data: PointerData<V>) -> (result: Self)
        ensures
            result.current.addr == data.addr,
            result.generation == 0,
    {
        PointerState {
            current: data,
            generation: 0,
        }
    }

    pub fn is_same_addr(&self, other: &Self) -> (result: bool)
        ensures
            result <==> self.current.addr == other.current.addr,
    {
        self.current.addr == other.current.addr
    }

    pub fn is_newer_than(&self, other: &Self) -> (result: bool)
        ensures
            result <==> self.generation > other.generation,
    {
        self.generation > other.generation
    }
}

impl<V: PartialEq> PointerState<V> {
    pub fn write(&mut self, new_value: Option<V>)
        requires
            old(self).generation < u64::MAX,
        ensures
            self.current.spec_write_rel(&old(self).current, new_value),
            self.generation == old(self).generation + 1,
    {
        self.current.value = new_value;
        self.generation = self.generation + 1;
    }

    pub fn update_flags(&mut self, new_flags: u32)
        requires
            old(self).generation < u64::MAX,
        ensures
            self.current.only_flags_updated(&old(self).current),
            self.current.flags == new_flags,
            self.generation == old(self).generation + 1,
    {
        self.current.flags = new_flags;
        self.generation = self.generation + 1;
    }
}

fn test_pointer_data_relations() {
    let data1 = PointerData::new(0x1000, Some(42u64), 0x1);
    let is_assigned = data1.check_assigned();

    let data2 = PointerData::new(0x1000, Some(42u64), 0x1);
    let equal = data1.check_sw_eq(&data2);

    let data3 = PointerData::new(0x1000, Some(100u64), 0x1);
    let only_val = data3.check_only_val_updated(&data1);

    let data4 = PointerData::new(0x1000, Some(42u64), 0x2);
    let only_flags = data4.check_only_flags_updated(&data1);

    let read_rel = data1.check_spec_read_rel(42u64);

    let mut state = PointerState::new(PointerData::new(0x2000, None, 0));
    state.write(Some(77u64));
    state.update_flags(0x4);

    let newer = state.is_newer_than(&PointerState::new(PointerData::new(0x2000, None, 0)));
}

} // verus!

fn main() {
    test_pointer_data_relations();
}
