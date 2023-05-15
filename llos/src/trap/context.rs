use riscv::register::sstatus::{self,Sstatus,SPP};

#[repr(C)]
pub struct TrapContext{
    pub x:[usize;32],
    pub sstatus:Sstatus,
    pub sepc:usize,
}

impl TrapContext{
    pub fn set_sp(&mut self,sp:usize){
        self.x[2]=sp;
    }

    pub fn app_init_context(entry:usize,sp:usize)->Self{
        //获取sstatus
        let mut sstatus=sstatus::read();
        //修改初始化特权级
        sstatus.set_spp(SPP::User);
        let mut cx=Self{
            x:[0;32],
            sstatus,
            sepc:entry,//指派的entry
        };
        cx.set_sp(sp); //指派的sp
        cx
    }
}
