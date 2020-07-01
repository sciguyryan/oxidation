bitflags! {
    #[derive(Default)]
    pub struct MemoryAccess: u8 {
        const N = 1 << 0;
        const R = 1 << 1;
        const W = 1 << 2;
        const PR = 1 << 3;
        const PW = 1 << 3;
    }
}

enum AccessType {
    Read,
    Write,
}

pub struct Memory {
    base_size: u32,
    stack_start: u32,
    stack_end: u32,
    stack_pointer: u32,
    data: Vec<i8>,
    memory_regions: Vec<MemoryRegion>,
    memory_seq_id: u32,
}

pub struct MemoryRegion {
    pub start: u32,
    pub end: u32,
    pub access: MemoryAccess,
    pub seq_id: u32,
    pub name: String,
}

impl Memory {
    pub fn new(main_memory_size: u32, stack_capacity: u32) -> Self {
        // There are 4 bytes in a 32-bit integer;
        let stack_size = stack_capacity * 4;

        // The final memory size is equal to the base memory
        // capacity plus the stack capacity.
        let memory_capacity = main_memory_size + stack_size;

        // The stack memory region will always be at the end
        // of the system memory.
        let stack_start = main_memory_size;
        let stack_end = memory_capacity;

        let mut mem = Self {
            base_size: memory_capacity,
            stack_start,
            stack_end,
            stack_pointer: stack_end,
            data: Vec::with_capacity(memory_capacity as usize),
            memory_regions: Vec::with_capacity(100),
            memory_seq_id: 0,
        };

        // The stack memory region should be marked
        // as public read, private write.
        // The only methods directly modifying it should be done
        // by the system.
        mem.add_memory_region(
            0,
            memory_capacity - 1,
            MemoryAccess::R | MemoryAccess::W,
            "Root".to_string(),
        );
        mem.add_memory_region(
            stack_start,
            stack_end,
            MemoryAccess::R | MemoryAccess::PW,
            "Stack".to_string(),
        );

        mem
    }

    pub fn len(&self) -> usize {
        self.data.capacity()
    }

    fn add_memory_region(&mut self, start: u32, end: u32, access: MemoryAccess, name: String) {
        let region = MemoryRegion::new(start, end, access, self.memory_seq_id, name);
        self.memory_regions.push(region);

        // Ensure that the sequence ID is never reused.
        self.memory_seq_id += 1;
    }
}

impl MemoryRegion {
    pub fn new(start: u32, end: u32, access: MemoryAccess, seq_id: u32, name: String) -> Self {
        Self {
            start,
            end,
            access,
            seq_id,
            name,
        }
    }
}
