// Variation: Hypercall Codes and Constants
// From: source/verismo/src/mshyper/mod.rs
// Demonstrates: Hypercall operation codes and register IDs

use vstd::prelude::*;

verus! {

pub const HVCALL_SET_VP_REGISTERS: u32 = 0x0051;
pub const HVCALL_GET_VP_REGISTERS: u32 = 0x0050;
pub const HVCALL_ENABLE_VP_VTL: u32 = 0x000f;
pub const HVCALL_START_VIRTUAL_PROCESSOR: u32 = 0x0099;

pub const HV_REGISTER_VSM_PARTITION_CONFIG: u32 = 0x000d0007;
pub const HV_X64_REGISTER_SEV_CONTROL: u32 = 0x00090040;
pub const HV_REGISTER_GUEST_VSM_PARTITION_CONFIG: u32 = 0x000D0008;

pub const HV_PARTITION_ID_SELF: u64 = 0xffff_ffff_ffff_ffff;
pub const HV_VP_INDEX_SELF: u32 = 0xffff_fffe;

pub fn is_set_vp_registers(code: u32) -> (result: bool)
    ensures
        result <==> code == HVCALL_SET_VP_REGISTERS,
{
    code == HVCALL_SET_VP_REGISTERS
}

pub fn is_get_vp_registers(code: u32) -> (result: bool)
    ensures
        result <==> code == HVCALL_GET_VP_REGISTERS,
{
    code == HVCALL_GET_VP_REGISTERS
}

pub fn is_enable_vp_vtl(code: u32) -> (result: bool)
    ensures
        result <==> code == HVCALL_ENABLE_VP_VTL,
{
    code == HVCALL_ENABLE_VP_VTL
}

pub fn is_start_virtual_processor(code: u32) -> (result: bool)
    ensures
        result <==> code == HVCALL_START_VIRTUAL_PROCESSOR,
{
    code == HVCALL_START_VIRTUAL_PROCESSOR
}

pub fn is_self_partition_id(id: u64) -> (result: bool)
    ensures
        result <==> id == HV_PARTITION_ID_SELF,
{
    id == HV_PARTITION_ID_SELF
}

pub fn is_self_vp_index(index: u32) -> (result: bool)
    ensures
        result <==> index == HV_VP_INDEX_SELF,
{
    index == HV_VP_INDEX_SELF
}

pub fn is_vsm_partition_config(reg: u32) -> (result: bool)
    ensures
        result <==> reg == HV_REGISTER_VSM_PARTITION_CONFIG,
{
    reg == HV_REGISTER_VSM_PARTITION_CONFIG
}

pub fn is_valid_hypercall(code: u32) -> (result: bool)
    ensures
        result <==> (code == HVCALL_SET_VP_REGISTERS
                  || code == HVCALL_GET_VP_REGISTERS
                  || code == HVCALL_ENABLE_VP_VTL
                  || code == HVCALL_START_VIRTUAL_PROCESSOR),
{
    code == HVCALL_SET_VP_REGISTERS
        || code == HVCALL_GET_VP_REGISTERS
        || code == HVCALL_ENABLE_VP_VTL
        || code == HVCALL_START_VIRTUAL_PROCESSOR
}

pub fn get_hypercall_category(code: u32) -> (result: u8)
    ensures
        result <= 2,
{
    if code == HVCALL_SET_VP_REGISTERS || code == HVCALL_GET_VP_REGISTERS {
        0  // Register operations
    } else if code == HVCALL_ENABLE_VP_VTL {
        1  // VTL operations
    } else if code == HVCALL_START_VIRTUAL_PROCESSOR {
        2  // VP operations
    } else {
        0  // Unknown
    }
}

fn test_hypercall_codes() {
    let is_set = is_set_vp_registers(HVCALL_SET_VP_REGISTERS);
    let is_get = is_get_vp_registers(HVCALL_GET_VP_REGISTERS);
    let is_enable = is_enable_vp_vtl(HVCALL_ENABLE_VP_VTL);
    let is_start = is_start_virtual_processor(HVCALL_START_VIRTUAL_PROCESSOR);

    let self_partition = is_self_partition_id(HV_PARTITION_ID_SELF);
    let self_vp = is_self_vp_index(HV_VP_INDEX_SELF);

    let is_vsm = is_vsm_partition_config(HV_REGISTER_VSM_PARTITION_CONFIG);

    let valid1 = is_valid_hypercall(HVCALL_SET_VP_REGISTERS);
    let valid2 = is_valid_hypercall(0x1234);

    let cat1 = get_hypercall_category(HVCALL_SET_VP_REGISTERS);
    let cat2 = get_hypercall_category(HVCALL_ENABLE_VP_VTL);
}

} // verus!

fn main() {
    test_hypercall_codes();
}
