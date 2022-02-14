use lang_3bc_sys::{app_3bc_s, driver_interrupt, driver_power_init, ds_program_fifo_line_add};
use std::ptr::null_mut;
unsafe fn lang_3bc_write_str(vm: *mut app_3bc_s, content: &str) {
    for chr in content.chars() {
        match chr {
            ' ' => ds_program_fifo_line_add(vm, 0b101, 0b000, 0x20),
            '\n' => ds_program_fifo_line_add(vm, 0b101, 0b000, 0x0a),
            chr => ds_program_fifo_line_add(vm, 0b101, 0b000, chr as _),
        }
    }
}
fn main() {
    unsafe {
        let vm = driver_power_init(0, null_mut());
        // MODE NILL MODE_STRING
        ds_program_fifo_line_add(vm, 0b111, 0b000, 2);
        lang_3bc_write_str(vm, "Hello World\n");

        while driver_interrupt(vm) {
            if vm.as_mut().unwrap().state == 1 {
                break;
            }
        }
    }
}
