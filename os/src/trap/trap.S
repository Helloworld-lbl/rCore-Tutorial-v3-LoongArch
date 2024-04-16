.altmacro
.macro SAVE_GP n
    st.d $r\n, $sp, \n*8
.endm
.macro LOAD_GP n
    ld.d $r\n, $sp, \n*8
.endm
    .section .text.trampoline
    .globl __alltraps
    .globl __restore
    .align 2
__alltraps:
    csrwr $sp, 0x30
    # now sp->kernel stack, save0->user stack
    # allocate a TrapContext on kernel stack
    addi.d $sp, $sp, -34*8
    # save general-purpose registers
    st.d $r1, $sp, 1*8
    # skip tp(r2), application does not use it
    # skip sp(r3), we will save it later
    # save r4~r31
    .set n, 4
    .rept 28
        SAVE_GP %n
        .set n, n+1
    .endr
    # we can use t0/t1/t2 freely, because they were saved on kernel stack
    csrrd $t0, 0x1
    csrrd $t1, 0x6
    st.d $t0, $sp, 32*8
    st.d $t1, $sp, 33*8
    # read user stack from save0 and save it on the kernel stack
    csrrd $r2, 0x30
    st.d $r2, $sp, 3*8
    # load kernel_pgdl into t0
    ld.d $t0, $sp, 34*8
    # load trap_handler into t1
    ld.d $t1, $sp, 35*8
    # switch to kernel space
    csrwr $t0, 0x19
    invtlb 0x0, $r0, $r0
    # jump to trap_handler
    move $a0, $sp
    jirl $r0, $t1, 0x0

__restore:
    # a0: user space token
    csrwr $a0, 0x19
    invtlb 0x0, $r0, $r0
    # now sp->kernel stack, start restoring based on it
    # restore prmd/era
    ld.d $t0, $sp, 32*8
    ld.d $t1, $sp, 33*8
    ld.d $t2, $sp, 3*8
    csrwr $t0, 0x1
    csrwr $t1, 0x6
    csrwr $t2, 0x30
    # restore general-purpuse registers except sp/tp
    ld.d $r1, $sp, 1*8
    .set n, 4
    .rept 28
        LOAD_GP %n
        .set n, n+1
    .endr
    # release TrapContext on kernel stack
    addi.d $sp, $sp, 34*8
    # now sp->kernel stack, save0->user stack
    csrwr $sp, 0x30
    ertn

