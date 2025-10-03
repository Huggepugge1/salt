const VGA: *mut i8 = 0xB8000 as *mut i8;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn _salt_std_io_vga_print(mut string: *const i8) {
    unsafe {
        let mut i = 0;
        while *string != 0 && i < 25 * 80 {
            *VGA.offset(i) = *string;
            string = string.add(1);
            i += 1;
        }
    }
}
