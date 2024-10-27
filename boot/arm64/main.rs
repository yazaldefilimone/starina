#![no_std]
#![no_main]

use core::arch::global_asm;

use ftl_inlinedvec::InlinedVec;
use ftl_kernel::boot::BootInfo;
use ftl_kernel::boot::FreeMem;
use ftl_kernel::cpuvar::CpuId;
use ftl_utils::byte_size::ByteSize;

global_asm!(include_str!("boot.S"));

extern "C" {
    static __bss: u8;
    static __bss_end: u8;
    static __free_ram: u8;
    static __free_ram_end: u8;
}

#[no_mangle]
unsafe extern "C" fn arm64_boot() -> ! {
    let bss_start = &__bss as *const _ as usize;
    let bss_end = &__bss_end as *const _ as usize;
    let free_ram = &__free_ram as *const _ as usize;
    let free_ram_end = &__free_ram_end as *const _ as usize;

    // Clear bss section.
    core::ptr::write_bytes(bss_start as *mut u8, 0, bss_end - bss_start);

    let mut free_mems = InlinedVec::<FreeMem, 8>::new();
    free_mems
        .try_push(FreeMem {
            start: free_ram,
            size: ByteSize(free_ram_end - free_ram),
        })
        .expect("too many free mems");

    // > For guests booting as “bare-metal” (any other kind of boot), the DTB
    // > is at the start of RAM (0x4000_0000)
    // >
    // > https://www.qemu.org/docs/master/system/arm/virt.html
    let dtb_addr = 0x4000_0000;

    ftl_kernel::boot::boot(
        CpuId::new(0 /* TODO: support multi-processors */),
        BootInfo {
            free_mems,
            cmdline: None,
            dtb_addr: Some(dtb_addr as *const u8),
        },
    );
}
