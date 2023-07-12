/**
* This is the definition of the built-in object in strawberry, implemented using inline assembly, please do not modify!!
* @author: https://github.com/mumu12641
* @update: 2023/7/12
*/

class Int inherits Object {
    val : PrimSlot;
    public fn to_string() -> String {
        // stack will be aligned to 16 bit
        __asm__("addq $8, %rsp"); 
        __asm__("movq $32, %rdi");
        __asm__("call malloc");
        __asm__("pushq %rax");
        __asm__("movq %rax, %rdi");
        __asm__("movq %d, %rsi");
        __asm__("movq INT_CONST_VAL_OFFSET(%rbx), %rdx");
        __asm__("call sprintf");
        __asm__("pushq $String_prototype");
        __asm__("call Object.malloc");
        __asm__("addq $8, %rsp");
        __asm__("call String.init");
        __asm__("popq %rdi");
        __asm__("movq %rdi, STRING_CONST_VAL_OFFSET(%rax)");
        __asm__("movq $32, 32(%rax)");
        // this is f**king stupid!! But it works, that's ok.
        __asm__("popq %rbx");
        __asm__("popq %rbp");
        __asm__("ret");
        return "";
    };
};