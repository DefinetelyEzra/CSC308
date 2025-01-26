#![no_std]
#![no_main]

mod writer;  // Import the writer module
use writer::FrameBufferWriter;  // Import the FrameBufferWriter struct
use core::fmt::Write;  // Import the Write trait for formatting output

use bootloader_api::config::Mapping;
use bootloader_api::entry_point;
use bootloader_api::BootInfo;
use core::panic::PanicInfo;
use x86_64::instructions::hlt;

pub static BOOTLOADER_CONFIG: bootloader_api::BootloaderConfig = {
    let mut config = bootloader_api::BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config.kernel_stack_size = 100 * 1024; // 100 KiB
    config
};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        hlt();
    }
}

entry_point!(my_entry_point, config = &BOOTLOADER_CONFIG);

fn my_entry_point(boot_info: &'static mut BootInfo) -> ! {
    // Get framebuffer information and buffer
    let frame_buffer_info = boot_info.framebuffer.as_mut().unwrap().info();
    let buffer = boot_info.framebuffer.as_mut().unwrap().buffer_mut();

    // Create a FrameBufferWriter instance
    let mut frame_buffer_writer = FrameBufferWriter::new(buffer, frame_buffer_info);

    // Write formatted text to the framebuffer
    writeln!(frame_buffer_writer, "Testing testing {} and {}", 1, 4.0/2.0).unwrap();

    loop {
        hlt();
    }
}