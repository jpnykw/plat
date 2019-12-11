	.section	__TEXT,__text,regular,pure_instructions
	.macosx_version_min 10, 14
	.globl	_main                   ## -- Begin function main
	.p2align	4, 0x90
_main:                                  ## @main
	.cfi_startproc
## %bb.0:                               ## %entry
	pushq	%rax
	.cfi_def_cfa_offset 16
	movl	$72, %edi
	callq	_putchar
	movl	$101, %edi
	callq	_putchar
	movl	$108, %edi
	callq	_putchar
	movl	$108, %edi
	callq	_putchar
	movl	$111, %edi
	callq	_putchar
	movl	$32, %edi
	callq	_putchar
	movl	$87, %edi
	callq	_putchar
	movl	$111, %edi
	callq	_putchar
	movl	$114, %edi
	callq	_putchar
	movl	$108, %edi
	callq	_putchar
	movl	$100, %edi
	callq	_putchar
	movl	$10, %edi
	callq	_putchar
	xorl	%eax, %eax
	popq	%rcx
	retq
	.cfi_endproc
                                        ## -- End function

.subsections_via_symbols
