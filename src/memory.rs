unsafe extern "C" {
    pub static __stack_top: usize;
    pub static __free_ram: usize;
    pub static __free_ram_end: usize;
}

const PAGE_SIZE: usize = 4096;

static mut TOTAL_PAGES: usize = 0;

pub fn init_memory() {
    unsafe {
        TOTAL_PAGES = (&__free_ram_end as *const usize as usize
            - &__free_ram as *const usize as usize)
            / PAGE_SIZE;
    }
}

pub fn alloc_pages(num_pages: usize) -> usize {
    // TODO: memset
    static mut USED_PAGES: usize = 0;

    let current_free: usize;

    unsafe {
        current_free = USED_PAGES;

        USED_PAGES += num_pages;

        if USED_PAGES > TOTAL_PAGES {
            panic!("Out of memory!");
        }
    }

    current_free * PAGE_SIZE + unsafe { &__free_ram as *const usize as usize }
}
