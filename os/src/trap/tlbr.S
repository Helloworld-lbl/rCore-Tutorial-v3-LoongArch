    .section .tlbrentry
    .globl __tlbrentry
    .align 2
__tlbrentry:
    csrwr $t0, 0x8b
    csrrd $t0, 0x1b
    lddir $t0, $t0, 3
    srli.w $t0, $t0, 12
    slli.w $t0, $t0, 12
    lddir $t0, $t0, 1
    srli.w $t0, $t0, 12
    slli.w $t0, $t0, 12
    ldpte $t0, 0
    ldpte $t0, 1
    tlbfill
    csrrd $t0, 0x8b
    ertn