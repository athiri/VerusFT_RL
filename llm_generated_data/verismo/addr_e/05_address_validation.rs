// Variation: Address Validation
// From: source/verismo/src/addr_e/addr_interface.rs
// Demonstrates: Address validity checking with size bounds

use vstd::prelude::*;

verus! {

pub const VM_MEM_SIZE: usize = 0x4000_0000; // 1GB
pub const VM_PAGE_NUM: usize = VM_MEM_SIZE / 0x1000;

pub fn check_valid_addr(addr: usize, size: usize) -> (result: bool)
    ensures
        result <==> (size <= VM_MEM_SIZE && addr <= VM_MEM_SIZE - size),
{
    size <= VM_MEM_SIZE && addr <= VM_MEM_SIZE - size
}

pub fn check_valid_page_num(page: usize, num_pages: usize) -> (result: bool)
    ensures
        result <==> (num_pages <= VM_PAGE_NUM && page <= VM_PAGE_NUM - num_pages),
{
    num_pages <= VM_PAGE_NUM && page <= VM_PAGE_NUM - num_pages
}

pub fn addr_fits_in_mem(addr: usize) -> (result: bool)
    ensures
        result <==> addr < VM_MEM_SIZE,
{
    addr < VM_MEM_SIZE
}

pub fn region_fits_in_mem(start: usize, size: usize) -> (result: bool)
    ensures
        result <==> (size <= VM_MEM_SIZE && start <= VM_MEM_SIZE - size),
{
    check_valid_addr(start, size)
}

pub fn safe_addr_add(addr: usize, offset: usize) -> (result: Option<usize>)
    ensures
        match result {
            Some(new_addr) => new_addr == addr + offset && new_addr <= VM_MEM_SIZE,
            None => offset > VM_MEM_SIZE || addr > VM_MEM_SIZE - offset,
        },
{
    if offset <= VM_MEM_SIZE && addr <= VM_MEM_SIZE - offset {
        Some(addr + offset)
    } else {
        None
    }
}

pub fn addr_range_valid(start: usize, end: usize) -> (result: bool)
    requires
        start <= end,
    ensures
        result <==> (start < VM_MEM_SIZE && end <= VM_MEM_SIZE),
{
    start < VM_MEM_SIZE && end <= VM_MEM_SIZE
}

pub fn normalize_addr(addr: usize) -> (result: usize)
    ensures
        result <= VM_MEM_SIZE,
        addr < VM_MEM_SIZE ==> result == addr,
        addr >= VM_MEM_SIZE ==> result == VM_MEM_SIZE,
{
    if addr < VM_MEM_SIZE {
        addr
    } else {
        VM_MEM_SIZE
    }
}

pub fn normalize_size(start: usize, size: usize) -> (result: usize)
    ensures
        result <= VM_MEM_SIZE,
{
    if start >= VM_MEM_SIZE {
        0
    } else if size < VM_MEM_SIZE - start {
        size
    } else {
        VM_MEM_SIZE - start
    }
}

fn test_address_validation() {
    let valid1 = check_valid_addr(0x1000, 0x2000);
    assert(valid1);

    let invalid1 = check_valid_addr(VM_MEM_SIZE - 0x100, 0x200);
    assert(!invalid1);

    let valid_page1 = check_valid_page_num(100, 10);
    assert(valid_page1);

    let fits1 = addr_fits_in_mem(0x1000);
    assert(fits1);

    let fits2 = addr_fits_in_mem(VM_MEM_SIZE);
    assert(!fits2);

    let region_fits = region_fits_in_mem(0x1000, 0x2000);
    assert(region_fits);

    let safe_add1 = safe_addr_add(0x1000, 0x2000);
    assert(safe_add1.is_some());
    assert(safe_add1.unwrap() == 0x3000);

    let safe_add2 = safe_addr_add(VM_MEM_SIZE - 0x100, 0x200);
    assert(safe_add2.is_none());

    let range_valid = addr_range_valid(0x1000, 0x3000);
    assert(range_valid);

    let normalized = normalize_addr(0x1000);
    assert(normalized == 0x1000);

    let normalized2 = normalize_addr(VM_MEM_SIZE + 0x1000);
    assert(normalized2 == VM_MEM_SIZE);

    let size_norm = normalize_size(0x1000, 0x2000);
    assert(size_norm <= VM_MEM_SIZE);
}

} // verus!

fn main() {
    test_address_validation();
}
