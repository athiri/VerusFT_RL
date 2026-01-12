// Variation: Register Value Management
// From: source/verismo/src/registers/msr_perm_s.rs (RegisterPermValue)
// Demonstrates: Register value tracking with CPU, ID, and shared flags

use vstd::prelude::*;

verus! {

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum RegisterId {
    MSR(u32),
    GeneralPurpose(u8),
    Control(u8),
}

pub struct RegisterValue<T> {
    pub cpu: usize,
    pub id: RegisterId,
    pub shared: bool,
    pub value: T,
}

impl<T> RegisterValue<T> {
    pub fn new(cpu: usize, id: RegisterId, shared: bool, value: T) -> (result: Self)
        ensures
            result.cpu == cpu,
            result.id == id,
            result.shared == shared,
            result.value == value,
    {
        RegisterValue { cpu, id, shared, value }
    }

    pub open spec fn shared_spec(&self) -> bool {
        self.shared
    }

    pub fn is_shared(&self) -> (result: bool)
        ensures
            result == self.shared_spec(),
    {
        self.shared
    }

    pub open spec fn value_spec(&self) -> T {
        self.value
    }

    pub fn get_value(&self) -> (result: &T)
        ensures
            *result == self.value_spec(),
    {
        &self.value
    }

    pub fn get_cpu(&self) -> (result: usize)
        ensures
            result == self.cpu,
    {
        self.cpu
    }

    pub fn get_id(&self) -> (result: RegisterId)
        ensures
            result == self.id,
    {
        self.id
    }
}

impl<T: PartialEq> RegisterValue<T> {
    pub open spec fn spec_write_value(&self, prev: &Self, val: T) -> bool {
        &&& prev.cpu == self.cpu
        &&& prev.id == self.id
        &&& prev.shared == self.shared
        &&& self.value == val
    }

    pub open spec fn registers_match(&self, other: &Self) -> bool {
        self.cpu == other.cpu && match (self.id, other.id) {
            (RegisterId::MSR(a), RegisterId::MSR(b)) => a == b,
            (RegisterId::GeneralPurpose(a), RegisterId::GeneralPurpose(b)) => a == b,
            (RegisterId::Control(a), RegisterId::Control(b)) => a == b,
            _ => false,
        }
    }

    pub fn is_same_register(&self, other: &Self) -> (result: bool)
        ensures
            result <==> self.registers_match(other),
    {
        if self.cpu != other.cpu {
            return false;
        }
        match (self.id, other.id) {
            (RegisterId::MSR(a), RegisterId::MSR(b)) => a == b,
            (RegisterId::GeneralPurpose(a), RegisterId::GeneralPurpose(b)) => a == b,
            (RegisterId::Control(a), RegisterId::Control(b)) => a == b,
            _ => false,
        }
    }

    pub fn has_same_config(&self, other: &Self) -> (result: bool)
    {
        self.is_same_register(other) && self.shared == other.shared
    }
}

pub struct RegisterState {
    pub cpu: usize,
    pub id: RegisterId,
    pub shared: bool,
    pub value: u64,
    pub modified: bool,
}

impl RegisterState {
    pub fn new(cpu: usize, id: RegisterId, shared: bool, value: u64) -> (result: Self)
        ensures
            result.cpu == cpu,
            result.value == value,
            !result.modified,
    {
        RegisterState { cpu, id, shared, value, modified: false }
    }

    pub fn write(&mut self, new_value: u64)
        ensures
            self.value == new_value,
            self.cpu == old(self).cpu,
            self.id == old(self).id,
            self.shared == old(self).shared,
            self.modified,
    {
        self.value = new_value;
        self.modified = true;
    }

    pub fn is_modified(&self) -> (result: bool)
        ensures
            result == self.modified,
    {
        self.modified
    }

    pub fn reset_modified(&mut self)
        ensures
            !self.modified,
            self.value == old(self).value,
            self.cpu == old(self).cpu,
    {
        self.modified = false;
    }
}

fn test_register_value() {
    let reg1 = RegisterValue::new(0, RegisterId::MSR(0xC0000080), false, 0x1000u64);
    let is_shared = reg1.is_shared();
    let cpu = reg1.get_cpu();
    let id = reg1.get_id();

    let reg2 = RegisterValue::new(0, RegisterId::MSR(0xC0000080), false, 0x2000u64);
    let same_reg = reg1.is_same_register(&reg2);
    let same_config = reg1.has_same_config(&reg2);

    let mut state = RegisterState::new(0, RegisterId::MSR(0xC0000080), false, 0x1000);
    state.write(0x2000);
    let is_modified = state.is_modified();
    state.reset_modified();
}

} // verus!

fn main() {
    test_register_value();
}
