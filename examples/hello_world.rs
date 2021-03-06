use lang_3bc_sys::{app_3bc_s, driver_interrupt, driver_power_init, ds_program_fifo_line_add};
use std::ptr::null_mut;
const NILL: u16 = 0;
const MODE: u8 = 0b111;
const MODE_STRING: u8 = 2;
const STRC: u8 = 0b101;

unsafe fn lang_3bc_write_str(vm: *mut app_3bc_s, content: &str) {
    for chr in content.chars() {
        match chr {
            ' ' => ds_program_fifo_line_add(vm, STRC, NILL, 0x20),
            '\n' => ds_program_fifo_line_add(vm, STRC, NILL, 0x0a),
            chr => ds_program_fifo_line_add(vm, STRC, NILL, chr as _),
        }
    }
}

fn main() {
    unsafe {
        let vm = driver_power_init(0, null_mut());

        ds_program_fifo_line_add(vm, MODE, NILL, MODE_STRING as _);
        lang_3bc_write_str(vm, "Hello World\n");

        while driver_interrupt(vm) {
            if vm.as_mut().unwrap().state == 1 {
                break;
            }
        }
    }
}
