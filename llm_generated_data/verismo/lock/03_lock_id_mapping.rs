// Variation: Lock ID and Pointer Mapping
// From: source/verismo/src/lock/spin_perm_s.rs (lockid_to_ptr, ptrid_to_lockid)
// Demonstrates: Converting between lock IDs and pointer IDs

use vstd::prelude::*;

verus! {

pub open spec fn lockid_to_ptrid(lockid: usize) -> usize {
    lockid
}

pub open spec fn ptrid_to_lockid(ptrid: usize) -> usize {
    ptrid
}

pub fn convert_lockid_to_ptr(lockid: usize) -> (result: usize)
    ensures
        result == lockid_to_ptrid(lockid),
{
    lockid
}

pub fn convert_ptrid_to_lock(ptrid: usize) -> (result: usize)
    ensures
        result == ptrid_to_lockid(ptrid),
{
    ptrid
}

pub fn lockid_roundtrip(lockid: usize) -> (result: usize)
    ensures
        result == lockid,
{
    let ptrid = convert_lockid_to_ptr(lockid);
    convert_ptrid_to_lock(ptrid)
}

pub fn ptrid_roundtrip(ptrid: usize) -> (result: usize)
    ensures
        result == ptrid,
{
    let lockid = convert_ptrid_to_lock(ptrid);
    convert_lockid_to_ptr(lockid)
}

pub fn ids_are_equal(lockid: usize, ptrid: usize) -> (result: bool)
    ensures
        result <==> lockid_to_ptrid(lockid) == ptrid,
{
    lockid == ptrid
}

pub fn get_ptr_from_lock(lockid: usize, offset: usize) -> (result: usize)
    requires
        lockid <= usize::MAX - offset,
    ensures
        result == lockid_to_ptrid(lockid) + offset,
{
    lockid + offset
}

pub fn get_lock_from_ptr(ptrid: usize, offset: usize) -> (result: usize)
    requires
        ptrid >= offset,
    ensures
        result == ptrid_to_lockid(ptrid) - offset,
{
    ptrid - offset
}

fn test_lock_id_mapping() {
    let lockid = 0x1000usize;
    let ptrid = 0x1000usize;

    let ptr = convert_lockid_to_ptr(lockid);
    let lock = convert_ptrid_to_lock(ptrid);

    let roundtrip1 = lockid_roundtrip(lockid);
    let roundtrip2 = ptrid_roundtrip(ptrid);

    let equal = ids_are_equal(lockid, ptrid);
    let not_equal = ids_are_equal(lockid, ptrid + 1);

    let ptr_with_offset = get_ptr_from_lock(lockid, 0x100);
    let lock_with_offset = get_lock_from_ptr(ptrid, 0x50);
}

} // verus!

fn main() {
    test_lock_id_mapping();
}
