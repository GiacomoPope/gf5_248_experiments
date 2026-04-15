.intel_syntax noprefix

#ifdef __APPLE__
.section __TEXT,__const
#else
.section .rodata
#endif

p:        .quad 0xffffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff, 0x04ffffffffffffff
p2:       .quad 0xfffffffffffffffe, 0xffffffffffffffff, 0xffffffffffffffff, 0x09ffffffffffffff
p_plus_1: .quad 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0500000000000000

#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack,"",@progbits
#endif

.text
.p2align 4,,15

#ifdef __APPLE__
.global _fp_add_asm
_fp_add_asm:
#else
.global fp_add_asm
fp_add_asm:
#endif
  xor    rax, rax
  mov    r8, [rsi]
  mov    r9, [rsi+8]
  mov    r10, [rsi+16]
  mov    r11, [rsi+24]
  add    r8, [rdx] 
  adc    r9, [rdx+8] 
  adc    r10, [rdx+16] 
  adc    r11, [rdx+24]
  mov    rax, r11
  shr    rax, 59
  neg    rax
  mov    rdx, [rip+p+24]
  and    rdx, rax
  sub    r8, rax
  sbb    r9, rax
  sbb    r10, rax
  sbb    r11, rdx
  
  mov    rax, r11
  shr    rax, 59
  neg    rax
  mov    rdx, [rip+p+24]
  and    rdx, rax
  sub    r8, rax
  sbb    r9, rax
  sbb    r10, rax
  sbb    r11, rdx
  mov    [rdi], r8
  mov    [rdi+8], r9 
  mov    [rdi+16], r10 
  mov    [rdi+24], r11
  ret

#ifdef __APPLE__
.global _fp_sub_asm
_fp_sub_asm:
#else
.global fp_sub_asm
fp_sub_asm:
#endif
  xor    rax, rax
  mov    r8, [rsi]
  mov    r9, [rsi+8]
  mov    r10, [rsi+16]
  mov    r11, [rsi+24]
  sub    r8, [rdx] 
  sbb    r9, [rdx+8] 
  sbb    r10, [rdx+16] 
  sbb    r11, [rdx+24]
  sbb    rax, 0
  
  mov    rdx, [rip+p+24]
  and    rdx, rax  
  add    r8, rax  
  adc    r9, rax
  adc    r10, rax  
  adc    r11, rdx 
  
  mov    rax, r11
  sar    rax, 59  
  mov    rdx, [rip+p+24]
  and    rdx, rax  
  add    r8, rax  
  adc    r9, rax
  adc    r10, rax  
  adc    r11, rdx

  mov    [rdi], r8
  mov    [rdi+8], r9 
  mov    [rdi+16], r10 
  mov    [rdi+24], r11 
  ret

///////////////////////////////////////////////////////////////// MACROS
// z = a x bi + z
// Inputs: base memory pointer M1 (a),
//         bi pre-stored in rdx,
//         accumulator z in [Z0:Z4]
// Output: [Z0:Z4]
// Temps:  regs T0:T1
/////////////////////////////////////////////////////////////////
.macro MULADD64x256 M1, Z0, Z1, Z2, Z3, Z4, T0, T1, C
    mulx   \T0, \T1, \M1     // A0*B0
    xor    \C, \C
    adox   \Z0, \T1
    adox   \Z1, \T0  
    mulx   \T0, \T1, 8\M1    // A0*B1
    adcx   \Z1, \T1
    adox   \Z2, \T0    
    mulx   \T0, \T1, 16\M1   // A0*B2
    adcx   \Z2, \T1
    adox   \Z3, \T0
    mulx   \T0, \T1, 24\M1   // A0*B3          
    adcx   \Z3, \T1
    adox   \Z4, \T0
    adc    \Z4, 0   
.endm

.macro MULADD64x64 M1, Z0, Z1, Z2, Z3, T0, T1
    mulx   \T0, \T1, \M1     // A0*B0
    xor    rax, rax
    adox   \Z2, \T1
    adox   \Z3, \T0
.endm

///////////////////////////////////////////////////////////////// MACRO
// z = a x b (mod p)
// Inputs: base memory pointers M0 (a), M1 (b)
//         bi pre-stored in rdx,
//         accumulator z in [Z0:Z4], pre-stores a0 x b
// Output: [Z0:Z4]
// Temps:  regs T0:T1
/////////////////////////////////////////////////////////////////
.macro FPMUL256x256 M0, M1, Z0, Z1, Z2, Z3, Z4, T0, T1           
    // [Z1:Z4] <- z = (z0 x p_plus_1 + z)/2^64
    mov    rdx, \Z0                 // rdx <- z0
    MULADD64x64 [rip+p_plus_1+24], \Z1, \Z2, \Z3, \Z4, \T0, \T1
    
    // [Z1:Z4, Z0] <- z = a01 x a1 + z 
    mov    rdx, 8\M0
    MULADD64x256 \M1, \Z1, \Z2, \Z3, \Z4, \Z0, \T0, \T1, \Z0
    // [Z2:Z4, Z0] <- z = (z0 x p_plus_1 + z)/2^64
    mov    rdx, \Z1                 // rdx <- z0
    MULADD64x64 [rip+p_plus_1+24], \Z2, \Z3, \Z4, \Z0, \T0, \T1
    
    // [Z2:Z4, Z0:Z1] <- z = a02 x a1 + z  
    mov    rdx, 16\M0
    MULADD64x256 \M1, \Z2, \Z3, \Z4, \Z0, \Z1, \T0, \T1, \Z1
    // [Z3:Z4, Z0:Z1] <- z = (z0 x p_plus_1 + z)/2^64
    mov    rdx, \Z2                // rdx <- z0
    MULADD64x64 [rip+p_plus_1+24], \Z3, \Z4, \Z0, \Z1, \T0, \T1
    
    // [Z3:Z4, Z0:Z2] <- z = a03 x a1 + z
    mov    rdx, 24\M0
    MULADD64x256 \M1, \Z3, \Z4, \Z0, \Z1, \Z2, \T0, \T1, \Z2
    // [Z4, Z0:Z2] <- z = (z0 x p_plus_1 + z)/2^64
    mov    rdx, \Z3                // rdx <- z0
    MULADD64x64 [rip+p_plus_1+24], \Z4, \Z0, \Z1, \Z2, \T0, \T1
.endm

#ifdef __APPLE__
.global _fp_mul_asm
_fp_mul_asm:
#else
.global fp_mul_asm
fp_mul_asm:
#endif
    push   r12
    push   r13 
    push   r14 
    mov    rcx, rdx 
     
    // [r8:r12] <- z = a x b0
    mov    rdx, [rcx]
    mulx   r9, r8, [rsi]
    xor    rax, rax 
    mulx   r10, r11, [rsi+8]
    adox   r9, r11        
    mulx   r11, r12, [rsi+16]
    adox   r10, r12        
    mulx   r12, r13, [rsi+24] 
    adox   r11, r13
    adox   r12, rax 

	FPMUL256x256 [rcx], [rsi], r8, r9, r10, r11, r12, r13, r14

    mov    [rdi], r12        
    mov    [rdi+8], r8         
    mov    [rdi+16], r9         
    mov    [rdi+24], r10  
    pop    r14
    pop    r13
    pop    r12
    ret
