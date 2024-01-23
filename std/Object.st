/**
* This is the definition of the built-in object in strawberry, implemented using inline assembly, please do not modify!!
* @author: https://github.com/mumu12641
* @update: 2023/7/12
*/

class Object{
    public fn print(val:String) -> void {
        __asm__("movq 24(%rbp), %rax");
        __asm__("pushq 32(%rax)");
        __asm__("movq 24(%rax), %rax");
        __asm__("pushq %rax");
        __asm__("movq $1, %rax");
        __asm__("movq $1, %rdi");
        __asm__("movq (%rsp), %rsi");
        __asm__("movq 8(%rsp), %rdx");
        __asm__("syscall");
        __asm__("addq $8, %rsp");
        __asm__("addq $8, %rsp");
        __asm__("movq %rbx, %rax");
        return;
    };

    // public fn to_string() -> String{
    //     return "";    
    // };
    
    public fn malloc() -> void {
        __asm__("movq 24(%rbp), %rax");
        __asm__("movq (%rax), %rdi");
        __asm__("call malloc");
        return ;
    };
};