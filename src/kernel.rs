#![no_std] // indicate that it does not depend on the standard library.
#![no_main] // indicate that it does not define a main function.

#[panic_handler] // the handler for panics, which just enters an infinite loop.
fn panic(_info: &core::panic::PanicInfo) -> ! 
{
    loop {}
}

#[no_mangle] //entry point for the kernel.
pub extern "C" fn _start() -> ! 
{
    // create a RAM disk
    let mut ramdisk = RamDisk::new(1024 * 1024);

    // format the RAM disk with ext4 filesystem
    let mut ext4_fs = Ext4FileSystem::new(&mut ramdisk).unwrap();
    ext4_fs.format().unwrap();
    
    // mount the ext4 filesystem
    let ext4_fs = open_file_system(&mut ramdisk, "ext4").unwrap();
    let mut root_dir = ext4_fs.root_dir().unwrap();
    
    // do something with the mounted filesystem
    let mut file = root_dir.create_file("hello.txt").unwrap();
    file.write("Hello, world!\n".as_bytes()).unwrap();

    
    let mut vfs = VFS::new();
    let fat32 = Fat32Driver::new();
    vfs.register_filesystem("fat32", Box::new(fat32));

    // defines a byte string containing the message to print.
    let hello = b"Hello, World!";

    // pointer to the VGA text buffer.
    let vga_buffer = 0xb8000 as *mut u8;
    
    // iterates over the bytes in the message.
    for (i, &byte) in hello.iter().enumerate() 
    {
        unsafe 
        {
            // writes each byte to the VGA buffer along with a color code to specify the color of the text
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb; // Light cyan on black
        }
    }

    // kernel enters infinite loop
    loop {}
}
