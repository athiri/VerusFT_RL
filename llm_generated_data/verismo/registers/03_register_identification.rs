// Variation: Register Identification
// From: source/verismo/src/registers/reg_trait_t.rs (AnyRegTrait, reg_id)
// Demonstrates: Register identification and type-safe operations

use vstd::prelude::*;

verus! {

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum RegType {
    MSR(u32),
    GeneralPurpose(u8),
    ControlReg(u8),
    DebugReg(u8),
}

pub struct RegisterInfo {
    pub reg_type: RegType,
    pub readable: bool,
    pub writable: bool,
}

impl RegisterInfo {
    pub fn new(reg_type: RegType, readable: bool, writable: bool) -> (result: Self)
        ensures
            result.reg_type == reg_type,
            result.readable == readable,
            result.writable == writable,
    {
        RegisterInfo { reg_type, readable, writable }
    }

    pub fn msr(msr_addr: u32) -> (result: Self)
        ensures
            result.reg_type == RegType::MSR(msr_addr),
            result.readable,
            result.writable,
    {
        RegisterInfo {
            reg_type: RegType::MSR(msr_addr),
            readable: true,
            writable: true,
        }
    }

    pub fn readonly_msr(msr_addr: u32) -> (result: Self)
        ensures
            result.reg_type == RegType::MSR(msr_addr),
            result.readable,
            !result.writable,
    {
        RegisterInfo {
            reg_type: RegType::MSR(msr_addr),
            readable: true,
            writable: false,
        }
    }

    pub open spec fn reg_id(&self) -> RegType {
        self.reg_type
    }

    pub fn get_reg_id(&self) -> (result: RegType)
        ensures
            result == self.reg_id(),
    {
        self.reg_type
    }

    pub fn can_read(&self) -> (result: bool)
        ensures
            result == self.readable,
    {
        self.readable
    }

    pub fn can_write(&self) -> (result: bool)
        ensures
            result == self.writable,
    {
        self.writable
    }

    pub open spec fn supports_operation(&self, is_write: bool) -> bool {
        if is_write {
            self.writable
        } else {
            self.readable
        }
    }

    pub fn check_operation(&self, is_write: bool) -> (result: bool)
        ensures
            result <==> self.supports_operation(is_write),
    {
        if is_write {
            self.writable
        } else {
            self.readable
        }
    }

    pub fn matches_type(&self, reg_type: RegType) -> (result: bool)
    {
        match (self.reg_type, reg_type) {
            (RegType::MSR(a), RegType::MSR(b)) => a == b,
            (RegType::GeneralPurpose(a), RegType::GeneralPurpose(b)) => a == b,
            (RegType::ControlReg(a), RegType::ControlReg(b)) => a == b,
            (RegType::DebugReg(a), RegType::DebugReg(b)) => a == b,
            _ => false,
        }
    }
}

pub struct RegisterAccessor<T> {
    pub info: RegisterInfo,
    pub phantom: core::marker::PhantomData<T>,
}

impl<T> RegisterAccessor<T> {
    pub fn new(info: RegisterInfo) -> (result: Self)
        ensures
            result.info.reg_type == info.reg_type,
            result.info.readable == info.readable,
            result.info.writable == info.writable,
    {
        RegisterAccessor {
            info,
            phantom: core::marker::PhantomData,
        }
    }

    pub open spec fn reg_id(&self) -> RegType {
        self.info.reg_type
    }

    pub fn get_reg_id(&self) -> (result: RegType)
        ensures
            result == self.reg_id(),
    {
        self.info.reg_type
    }

    pub open spec fn read_allowed(&self) -> bool {
        self.info.readable
    }

    pub open spec fn write_allowed(&self) -> bool {
        self.info.writable
    }

    pub fn can_read(&self) -> (result: bool)
        ensures
            result == self.read_allowed(),
    {
        self.info.readable
    }

    pub fn can_write(&self) -> (result: bool)
        ensures
            result == self.write_allowed(),
    {
        self.info.writable
    }
}

pub const MSR_EFER: u32 = 0xC0000080;
pub const MSR_STAR: u32 = 0xC0000081;
pub const MSR_LSTAR: u32 = 0xC0000082;
pub const MSR_GHCB_BASE: u32 = 0xC0010130;

pub fn create_efer_accessor() -> (result: RegisterAccessor<u64>)
    ensures
        result.reg_id() == RegType::MSR(MSR_EFER),
        result.read_allowed(),
        result.write_allowed(),
{
    RegisterAccessor::new(RegisterInfo::msr(MSR_EFER))
}

pub fn create_ghcb_accessor() -> (result: RegisterAccessor<u64>)
    ensures
        result.reg_id() == RegType::MSR(MSR_GHCB_BASE),
{
    RegisterAccessor::new(RegisterInfo::msr(MSR_GHCB_BASE))
}

fn test_register_identification() {
    let info = RegisterInfo::msr(MSR_EFER);
    let can_read = info.can_read();
    let can_write = info.can_write();
    let check_write = info.check_operation(true);

    let readonly = RegisterInfo::readonly_msr(0x00000000);
    let is_writable = readonly.can_write();

    let accessor = create_efer_accessor();
    let reg_id = accessor.get_reg_id();
    let readable = accessor.can_read();

    let matches = info.matches_type(RegType::MSR(MSR_EFER));
}

} // verus!

fn main() {
    test_register_identification();
}
