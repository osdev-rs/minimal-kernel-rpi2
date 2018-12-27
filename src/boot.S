// To keep this in the first portion of the binary.
.section ".text.boot"
 
// Make _start global.
.globl _start
 
// Entry point for the kernel.
// r15 -> should begin execution at 0x8000.
// r0 -> 0x00000000
// r1 -> 0x00000C42
// r2 -> 0x00000100 - start of ATAGS
// preserve these registers as argument for kernel_main
_start:
        ldr pc,reset_vector
        ldr pc,undefined_vector
	ldr pc,swi_vector
	ldr pc,prefetch_vector
	ldr pc,data_vector
	ldr pc,unused_vector
	ldr pc,irq_vector
	ldr pc,fiq_vector

reset_vector:      .word reset_asm_handler
undefined_vector:  .word undefined_asm_handler
swi_vector:        .word swi_asm_handler
prefetch_vector:   .word prefetch_asm_handler
data_vector:       .word data_asm_handler
unused_vector:     .word unused_asm_handler
irq_vector:        .word irq_asm_handler
fiq_vector:        .word fiq_asm_handler

reset_asm_handler:

;;@ Copy IVT at address 0 as it should be
        mov r0,#0x8000
        mov r1,#0x0000
	ldmia r0!,{r2,r3,r4,r5,r6,r7,r8,r9}
	stmia r1!,{r2,r3,r4,r5,r6,r7,r8,r9}
	ldmia r0!,{r2,r3,r4,r5,r6,r7,r8,r9}
	stmia r1!,{r2,r3,r4,r5,r6,r7,r8,r9}

        // Setup Stack
        // IRQ
        // (PSR_IRQ_MODE|PSR_FIQ_DIS|PSR_IRQ_DIS)
        mov r0,#0xD2
        msr cpsr_c,r0
        ldr sp, =__irq_stack_end__


        // (PSR_SYSTEM_MODE|PSR_FIQ_DIS|PSR_IRQ_DIS)
        mov r0,#0xDF
        msr cpsr_c,r0
        ldr sp, =__sys_stack_end__

	// Clear out bss.
	ldr r4, =__bss_start
	ldr r9, =__bss_end
	mov r5, #0
	mov r6, #0
	mov r7, #0
	mov r8, #0
	b       2f
 
1:
	// store multiple at r4.
	stmia r4!, {r5-r8}
 
	// If we are still below bss_end, loop.
2:
	cmp r4, r9
	blo 1b
 
	// Call kernel_main
	ldr r3, =kernel_main
	blx r3
 
	// halt
halt:
	wfe
	b halt

undefined_asm_handler:
        b undefined_asm_handler

swi_asm_handler:
        b swi_asm_handler

prefetch_asm_handler:
        b prefetch_asm_handler

data_asm_handler:
        b data_asm_handler

unused_asm_handler:
        b unused_asm_handler

irq_asm_handler:
        push {r0-r3,r12,lr}
        bl irq_handler
        pop {r0-r3,r12,lr}
        subs pc,lr,#4

fiq_asm_handler:
        b fiq_asm_handler
