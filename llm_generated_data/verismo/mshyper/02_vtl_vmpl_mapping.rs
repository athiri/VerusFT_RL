// Variation: VTL and VMPL Mapping
// From: source/verismo/src/mshyper/mod.rs (vtl_vmpl_map, get_vtl)
// Demonstrates: Virtual Trust Level to VM Permission Level mapping

use vstd::prelude::*;

verus! {

pub open spec fn spec_vtl_vmpl_map(vtl: u8, vmpl: u8) -> bool {
    ||| (vtl == 2 && vmpl == 0)
    ||| (vtl == 0 && vmpl != 0)
}

pub fn get_vtl_from_vmpl(vmpl: u8) -> (result: u8)
    ensures
        spec_vtl_vmpl_map(result, vmpl),
{
    if vmpl == 0 {
        2
    } else {
        0
    }
}

pub fn is_vtl2(vtl: u8) -> (result: bool)
    ensures
        result <==> vtl == 2,
{
    vtl == 2
}

pub fn is_vtl0(vtl: u8) -> (result: bool)
    ensures
        result <==> vtl == 0,
{
    vtl == 0
}

pub fn is_vmpl0(vmpl: u8) -> (result: bool)
    ensures
        result <==> vmpl == 0,
{
    vmpl == 0
}

pub fn is_valid_vmpl(vmpl: u8) -> (result: bool)
    ensures
        result <==> vmpl < 4,
{
    vmpl < 4
}

pub fn is_valid_vtl(vtl: u8) -> (result: bool)
    ensures
        result <==> (vtl == 0 || vtl == 1 || vtl == 2),
{
    vtl == 0 || vtl == 1 || vtl == 2
}

pub fn vtl_has_higher_privilege(vtl1: u8, vtl2: u8) -> (result: bool)
    ensures
        result <==> vtl1 > vtl2,
{
    vtl1 > vtl2
}

pub fn vmpl_has_higher_privilege(vmpl1: u8, vmpl2: u8) -> (result: bool)
    ensures
        result <==> vmpl1 < vmpl2,
{
    vmpl1 < vmpl2
}

pub fn check_vtl_vmpl_consistency(vtl: u8, vmpl: u8) -> (result: bool)
    ensures
        result <==> spec_vtl_vmpl_map(vtl, vmpl),
{
    (vtl == 2 && vmpl == 0) || (vtl == 0 && vmpl != 0)
}

fn test_vtl_vmpl_mapping() {
    let vtl_from_0 = get_vtl_from_vmpl(0);
    let vtl_from_1 = get_vtl_from_vmpl(1);
    let vtl_from_2 = get_vtl_from_vmpl(2);

    let is_2 = is_vtl2(2);
    let is_0 = is_vtl0(0);

    let vmpl_is_0 = is_vmpl0(0);
    let vmpl_is_1 = is_vmpl0(1);

    let valid_vmpl = is_valid_vmpl(3);
    let invalid_vmpl = is_valid_vmpl(4);

    let valid_vtl = is_valid_vtl(2);
    let invalid_vtl = is_valid_vtl(3);

    let vtl_higher = vtl_has_higher_privilege(2, 0);
    let vmpl_higher = vmpl_has_higher_privilege(0, 1);

    let consistent1 = check_vtl_vmpl_consistency(2, 0);
    let consistent2 = check_vtl_vmpl_consistency(0, 1);
    let inconsistent = check_vtl_vmpl_consistency(2, 1);
}

} // verus!

fn main() {
    test_vtl_vmpl_mapping();
}
