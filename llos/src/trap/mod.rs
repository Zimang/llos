mod context;

use crate::syscall::syscall;
use crate::task::{exit_current_and_run_next,suspenf_current_and_run_next};
use crate::timer::set_next_trigger;
use riscv::register::{
    mtvec::TrapMode,
    scause::{self,Exception,Interrupt,Trap},
    sie,stval,stvec,
};

core::arch::global_asm!(include_str!("trap.S"));

pub fn init(){
    extern "C"{
        fn __alltraps();
    }
    unsafe{
        stvec::write(__alltraps as usize,TrapMode::Direct);
    }
}

#[no_mangle]
pub fn tarp_handler(cx:&mut TrapContext)-> &mut TrapContext{
    let scause=scause::read();
    let stval=stval::read();
    match scause.cause(){
        Trap::Exception(Exception::UserEnvCall)=>{
            cx.sepc +=4;
            cx.x[10]=syscall(cx.x[17],[cx.x[10],cx.x[11],cx.x[12]]) as usize;
        }
        Trap::Exception(Exception::StoreFault)|Trap::Exception(Exception::StorePageFault)=>{
            error!("[kernel] PageFault in application,core dumped.");
            run_next_app();
        }
        Trap::Exception(Exception::IllegalInstruction)=>{
            error!("[kernel] IllpubegalInstruction in application,core dumped.");
            run_next_app();
        }
        _=>{
            panic!{
                "Unsupported trap{:?},stval={:#x}!",
                scause.cause(),
                stval
            };
        }
    }
}

pub use context::TrapContext;
