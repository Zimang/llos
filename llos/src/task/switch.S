.altmarco
.marco SACE_SN n
	sd s\n,(\n+2)*8(a0)
.endm
.marco LOAD_SN n
	ld s\n,(\n+2)*8(a1)
.endm
	.section .text
	.globl __switch
__switch:
	# __switch(
	#	current_task_cx_ptr: *mut TaskContext,	当前任务指针
	#	next_task_cn_ptr:  *const TaskContext   下一个任务指针
	# )
	# ao第一个参数,a1第二个参数
	# 保存当前任务指针
	sd sp,8(a0)
	# 保存ra,以及其他的通用寄存器
	sd ra,0(a0)
	.set n,0
	.rept 12
		SAVE_SN %n
		.set n,n+1
	.endr
	#加载下一任务
	ld ra,0(a1)
	.set n,0
	.rept 12
		LOAD_SN %n
		.set n,n+1
	.endr
	ld sp,8(a1)
	ret
