use core::arch::asm;

const SSIZE: isize = 48;

fn hello() -> ! {
    println!("wake up on a new stack");
    loop {}
}

#[derive(Debug, Default)]
#[repr(C)]
struct ThreadContext {
    rsp: u64,
}

unsafe fn gt_switch(new: *const ThreadContext) {
    asm!(
    "mov rsp, [{0} + 0x00]",
    "ret",
    in(reg) new,
    );
}

fn main() {
    let mut ctx = ThreadContext::default();
    let mut stack = vec![0_u8; SSIZE as usize];
    unsafe {
        let stack_bottom = stack.as_mut_ptr().offset(SSIZE);
        let sb_aligned = (stack_bottom as usize & !15) as *mut u8;
        for i in 0..SSIZE {
            println!("stack mem: {}, val: {}",
                     sb_aligned.offset(-i as isize) as usize,
                     *sb_aligned.offset(-i as isize))
        }
        std::ptr::write(sb_aligned.offset(-16) as *mut u64, hello as u64);
        ctx.rsp = sb_aligned.offset(-16) as u64;
        gt_switch(&mut ctx);
    }
}