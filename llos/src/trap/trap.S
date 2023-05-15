.altmarco
.marco SAVE_GP n
    sd x\n,\n*8(sp)
.endm
.marco LOAD_GP n
    ld x\n,\n*8(sp)
.endm
    .section .text
    .global __alltraps
    .global __restore
    .align 2
__alltraps:
    #交换内核与用户栈栈指针
    csrw sp,sscratch,sp
    #扩充34格内核栈
    addi sp ,sp,-34*8
    #保存通用寄存器
    sd x1,1*8(sp) #ra
    sd x3,3*8(sp) #gp
    .set n,5
    .rept 27
	SAVE_GP %n
	.set n,n+1
    .endr
    #保存csr,先将csr移至tep,temp移至sp
    csrr t0,sstatus
    csrr t1,sepc
    sd t0,32*8(sp)
    sd t1,33*8(sp)
    #保存user sp
    csrr t2,sscratch
    sd t2,2*8(sp)
    #传递内核栈地址，执行trap分发
    mv a0,sp
    call trap_handler

__restore:
    #csr寄存器
    ld t0,32*8(sp)
    ld t1,33*8(sp)
    ld t2,2*8(sp)
    csrw sstatus,t0
    csrw sepc,t1
    csrw sscratch,t2
    #存储通用寄存器
    ld x1,1*8*(sp)
    ld x3,3*8(sp)
    .set n,5
    .rept 27
	LOAD_GP %n
	.set n,n+1
    .endr
    #释放内存地址
    addi sp,sp,34*8
    #交换内核与用户栈指针
    csrrw sp,sscratch,sp
    sret
