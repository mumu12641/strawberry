# Strawberry
## TODO
* Math expression have no postion.

https://github.com/maekawatoshiki/vicis.git
https://gitlab.com/taricorp/llvm-sys.rs
https://github.com/cdisselkoen/llvm-ir
https://github.com/TheDan64/inkwell
https://github.com/mun-lang/lld-rs

使用 rbp 存函数帧
rsp 存栈
rbx 存 self 对象

第一次 malloc 的时候，应该返回在 rax 中是对象指针，先把 rax -> rbx，然后调用函数的时候，expr.function()，先计算 expr ，把 expr 的指针存到rax，然后调用的时候，被调用方会先保存调用方的 rbx，然后把 rax -> rbx ！！调用完把栈里的 rbx 恢复即可，rax还是函数的返回值