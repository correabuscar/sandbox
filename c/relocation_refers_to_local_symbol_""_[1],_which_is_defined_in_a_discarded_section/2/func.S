# define GET_PC_THUNK(reg) __x86.get_pc_thunk.reg
	
# define SETUP_PIC_REG(reg) \
  .ifndef GET_PC_THUNK(reg);						      \
  .section .gnu.linkonce.t.GET_PC_THUNK(reg),"ax",@progbits;		      \
  .globl GET_PC_THUNK(reg);						      \
  .hidden GET_PC_THUNK(reg);						      \
  .p2align 4;								      \
  .type GET_PC_THUNK(reg),@function;					      \
GET_PC_THUNK(reg):							      \
  movl (%esp), %e##reg;							      \
  ret;									      \
  .size GET_PC_THUNK(reg), . - GET_PC_THUNK(reg);			      \
  .previous;								      \
  .endif;								      \
	call GET_PC_THUNK(reg)

# define LOAD_PIC_REG(reg) \
  SETUP_PIC_REG(reg); addl $_GLOBAL_OFFSET_TABLE_, %e##reg

	.p2align 4
	.globl	FUNC
	.type FUNC,@function
FUNC:
	LOAD_PIC_REG(ax)
	ret
	.size FUNC, . - FUNC
