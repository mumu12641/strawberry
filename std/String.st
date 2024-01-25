// /**
// * This is the definition of the built-in object in strawberry, implemented using inline assembly, please do not modify!!
// * @author: https://github.com/mumu12641
// * @update: 2023/7/12
// */

// class String inherits Object {
//     // val : int;
//     len : int;
//     ptr : str;
//     public fn concat(dest:String, src:String) -> void {
//         // stack will be aligned to 16 bit
//         __asm__("addq $8, %rsp");
//         __asm__("movq $64, %rdi");
//         __asm__("call malloc");
//         __asm__("movq %rax, %r10");
//         __asm__("movq %r10, %rdi");
//         __asm__("movq 32(%rbp), %rax");
//         __asm__("movq STRING_CONST_VAL_OFFSET(%rax), %rsi");
//         __asm__("call strcpy");
//         __asm__("movq %r10, %rdi");
//         __asm__("movq 24(%rbp), %rax");
//         __asm__("movq STRING_CONST_VAL_OFFSET(%rax), %rsi");
//         __asm__("call strcat");
//         __asm__("pushq $String_prototype");
//         __asm__("call Object.malloc");
//         __asm__("addq $8, %rsp");
//         __asm__("call String.init");
//         __asm__("movq %r10, STRING_CONST_VAL_OFFSET(%rax)");
//         __asm__("movq $64, 32(%rax)");
//         __asm__("subq $8, %rsp");
//         return;
//     };

//     public fn to_string() -> String {
//         return self;
//     };

// };