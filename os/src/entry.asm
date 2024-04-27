    .section .text.entry
    .globl start
start:
    la $sp, boot_stack_top
    bl rust_main

    .section .bss.stack
    .globl boot_stack_lower_bound
boot_stack_lower_bound:
    .space 4096 * 16
    .globl boot_stack_top
boot_stack_top:
