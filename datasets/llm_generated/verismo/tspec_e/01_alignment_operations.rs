// Variation: Alignment Operations
// From: source/verismo/src/tspec_e/math/align_e.rs
// Demonstrates: Address alignment up and down to power-of-2 boundaries

use vstd::prelude::*;

verus! {

pub open spec fn is_power_of_2(n: int) -> bool {
    n > 0 && (n == 1 || n == 2 || n == 4 || n == 8 || n == 16 || n == 32 || n == 64 ||
              n == 128 || n == 256 || n == 512 || n == 1024 || n == 2048 || n == 4096 ||
              n == 8192 || n == 16384 || n == 32768 || n == 65536)
}

pub open spec fn spec_align_up(val: int, align: int) -> int
    recommends
        is_power_of_2(align),
{
    if val % align == 0 {
        val
    } else {
        val + (align - val % align)
    }
}

pub open spec fn spec_align_down(val: int, align: int) -> int
    recommends
        is_power_of_2(align),
{
    val - val % align
}

pub struct AlignedValue {
    pub value: u64,
    pub alignment: u64,
}

impl AlignedValue {
    pub fn new(value: u64, alignment: u64) -> (result: Self)
        ensures
            result.value == value,
            result.alignment == alignment,
    {
        AlignedValue { value, alignment }
    }

    pub open spec fn is_aligned(&self) -> bool {
        self.value as int % self.alignment as int == 0
    }

    pub open spec fn wf(&self) -> bool {
        &&& self.alignment > 0
        &&& is_power_of_2(self.alignment as int)
    }

    pub fn get_value(&self) -> (result: u64)
        ensures
            result == self.value,
    {
        self.value
    }

    pub fn get_alignment(&self) -> (result: u64)
        ensures
            result == self.alignment,
    {
        self.alignment
    }
}

pub struct AlignmentRange {
    pub start: u64,
    pub end: u64,
    pub alignment: u64,
}

impl AlignmentRange {
    pub fn new(start: u64, end: u64, alignment: u64) -> (result: Self)
        ensures
            result.start == start,
            result.end == end,
            result.alignment == alignment,
    {
        AlignmentRange { start, end, alignment }
    }

    pub open spec fn wf(&self) -> bool {
        &&& self.alignment > 0
        &&& is_power_of_2(self.alignment as int)
        &&& self.start <= self.end
    }

    pub open spec fn start_is_aligned(&self) -> bool {
        self.start as int % self.alignment as int == 0
    }

    pub open spec fn end_is_aligned(&self) -> bool {
        self.end as int % self.alignment as int == 0
    }

    pub open spec fn is_fully_aligned(&self) -> bool {
        self.start_is_aligned() && self.end_is_aligned()
    }

    pub open spec fn len(&self) -> int {
        self.end as int - self.start as int
    }

    pub open spec fn contains(&self, addr: u64) -> bool {
        self.start <= addr < self.end
    }

    pub fn get_start(&self) -> (result: u64)
        ensures
            result == self.start,
    {
        self.start
    }

    pub fn get_end(&self) -> (result: u64)
        ensures
            result == self.end,
    {
        self.end
    }

    pub fn get_alignment(&self) -> (result: u64)
        ensures
            result == self.alignment,
    {
        self.alignment
    }
}

pub struct PageAligned {
    pub addr: u64,
}

impl PageAligned {
    pub const PAGE_SIZE: u64 = 4096;

    pub fn new(addr: u64) -> (result: Self)
        ensures
            result.addr == addr,
    {
        PageAligned { addr }
    }

    pub open spec fn is_page_aligned(&self) -> bool {
        self.addr as int % Self::PAGE_SIZE as int == 0
    }

    pub open spec fn aligned_up(&self) -> int {
        spec_align_up(self.addr as int, Self::PAGE_SIZE as int)
    }

    pub open spec fn aligned_down(&self) -> int {
        spec_align_down(self.addr as int, Self::PAGE_SIZE as int)
    }

    pub fn get_addr(&self) -> (result: u64)
        ensures
            result == self.addr,
    {
        self.addr
    }

    pub open spec fn page_offset(&self) -> int {
        self.addr as int % Self::PAGE_SIZE as int
    }

    pub open spec fn wf(&self) -> bool {
        true
    }
}

pub struct SectorAligned {
    pub offset: u64,
}

impl SectorAligned {
    pub const SECTOR_SIZE: u64 = 512;

    pub fn new(offset: u64) -> (result: Self)
        ensures
            result.offset == offset,
    {
        SectorAligned { offset }
    }

    pub open spec fn is_sector_aligned(&self) -> bool {
        self.offset as int % Self::SECTOR_SIZE as int == 0
    }

    pub open spec fn aligned_up(&self) -> int {
        spec_align_up(self.offset as int, Self::SECTOR_SIZE as int)
    }

    pub open spec fn aligned_down(&self) -> int {
        spec_align_down(self.offset as int, Self::SECTOR_SIZE as int)
    }

    pub fn get_offset(&self) -> (result: u64)
        ensures
            result == self.offset,
    {
        self.offset
    }

    pub open spec fn sector_offset(&self) -> int {
        self.offset as int % Self::SECTOR_SIZE as int
    }

    pub open spec fn wf(&self) -> bool {
        true
    }
}

fn test_alignment_operations() {
    let aligned = AlignedValue::new(4096, 4096);
    let value = aligned.get_value();
    let alignment = aligned.get_alignment();

    let range = AlignmentRange::new(0, 8192, 4096);
    let start = range.get_start();
    let end = range.get_end();
    let range_align = range.get_alignment();

    let page = PageAligned::new(8192);
    let page_addr = page.get_addr();

    let sector = SectorAligned::new(1024);
    let sector_off = sector.get_offset();

    proof {
        assert(value == 4096);
        assert(alignment == 4096);
        assert(aligned.is_aligned());
        assert(is_power_of_2(4096));
        assert(aligned.wf());
        assert(start == 0);
        assert(end == 8192);
        assert(range_align == 4096);
        assert(range.start_is_aligned());
        assert(range.end_is_aligned());
        assert(range.is_fully_aligned());
        assert(range.len() == 8192);
        assert(range.contains(4096));
        assert(range.wf());
        assert(page_addr == 8192);
        assert(page.is_page_aligned());
        assert(page.page_offset() == 0);
        assert(page.wf());
        assert(sector_off == 1024);
        assert(sector.is_sector_aligned());
        assert(sector.sector_offset() == 0);
        assert(sector.wf());
        assert(spec_align_down(100, 64) == 64);
        assert(spec_align_up(100, 64) == 128);
    }
}

} // verus!

fn main() {
    test_alignment_operations();
}
