#![no_std]

use allocator::{BaseAllocator, ByteAllocator, PageAllocator};

/// Early memory allocator
/// Use it before formal bytes-allocator and pages-allocator can work!
/// This is a double-end memory range:
/// - Alloc bytes forward
/// - Alloc pages backward
///
/// [ bytes-used | avail-area | pages-used ]
/// |            | -->    <-- |            |
/// start       b_pos        p_pos       end
///
/// For bytes area, 'count' records number of allocations.
/// When it goes down to ZERO, free bytes-used area.
/// For pages area, it will never be freed!
///
pub struct EarlyAllocator<const SIZE: usize> {
    start: usize,
    size: usize,
    b_pos: usize,
    p_pos: usize,
}

impl<const SIZE: usize> EarlyAllocator<SIZE> {
    pub const fn new() -> Self {
        Self {
            start: 0,
            size: 0,
            b_pos: 0,
            p_pos: 0,
        }
    }
}

impl<const SIZE: usize> BaseAllocator for EarlyAllocator<SIZE> {
    fn init(&mut self, start: usize, size: usize) {
        self.start = start;
        self.size = size;
        self.b_pos = start;
        self.p_pos = start + size;
    }

    fn add_memory(&mut self, _start: usize, _size: usize) -> allocator::AllocResult {
        Err(allocator::AllocError::NoMemory)
    }
}

impl<const SIZE: usize> ByteAllocator for EarlyAllocator<SIZE> {
    fn alloc(
        &mut self,
        layout: core::alloc::Layout,
    ) -> allocator::AllocResult<core::ptr::NonNull<u8>> {
        if self.b_pos + layout.size() > self.p_pos {
            return Err(allocator::AllocError::NoMemory);
        }
        let ptr = self.b_pos;
        self.b_pos += layout.size();
        Ok(core::ptr::NonNull::new(ptr as *mut u8).unwrap())
    }

    fn dealloc(&mut self, pos: core::ptr::NonNull<u8>, layout: core::alloc::Layout) {
        let addr = pos.as_ptr() as usize;
        if addr < self.b_pos || addr >= self.p_pos {
            return;
        }
        self.b_pos = addr.wrapping_sub(layout.size());
    }

    fn total_bytes(&self) -> usize {
        self.size
    }

    fn used_bytes(&self) -> usize {
        self.b_pos - self.start
    }

    fn available_bytes(&self) -> usize {
        self.p_pos - self.b_pos
    }
}

impl<const SIZE: usize> PageAllocator for EarlyAllocator<SIZE> {
    const PAGE_SIZE: usize = SIZE;

    fn alloc_pages(
        &mut self,
        num_pages: usize,
        align_pow2: usize,
    ) -> allocator::AllocResult<usize> {
        if self.p_pos - num_pages * SIZE < self.b_pos {
            return Err(allocator::AllocError::NoMemory);
        }
        let ptr = self.p_pos - num_pages * SIZE;
        self.p_pos = ptr;
        Ok(ptr)
    }

    fn dealloc_pages(&mut self, pos: usize, num_pages: usize) {
        let addr = pos;
        if addr < self.b_pos || addr >= self.p_pos {
            return;
        }
        self.p_pos = addr + num_pages * SIZE;
    }

    fn total_pages(&self) -> usize {
        self.size / SIZE
    }

    fn used_pages(&self) -> usize {
        (self.start + self.size - self.p_pos) / SIZE
    }

    fn available_pages(&self) -> usize {
        (self.p_pos - self.b_pos) / SIZE
    }
}