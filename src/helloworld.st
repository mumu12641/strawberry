
class Main{
public fn main() -> int{
let a:int=0;
// let b:int = a + 1;
// if(a>1){
// a=1;
// }else{
// a=0;
// }
// a = 1+2;
while(a >1){
a=a+1;
}
return 0;
};
};
// 
// 
// 
// 
// @Main.main: int {
// //   a.0 = const 0;
//   a.1 = const 0;
//   jmp .branch0;
// .branch0:
//   t2.1 = a.1;
//   t4.1: Object = more t2.1 const 1;
//   br t4.1 .branch1 .branch2;
// .branch1:
//   t5.1 = a.1;
//   t7.1: Object = add t5.1 const 1;
// //   a.2 = t7.1;
// //   a.1 = a.2;
//     a.1 = t7.1
//   jmp .branch0;
// .branch2:
//   t8.0: int = const 0;
//   ret const 0
// }

// 先建立 def use 链，然后constant prop ，然后from ssa，然后 lvn 和 dce
// @main {
// .block.0:
//   t1.0: int = const 0;
//   a.0: int = id t1.0;
//   a.1: int = id a.0;
//   jmp .branch0;
// .branch0:
//   t2.1: int = id a.1;
//   t3.1: int = const 1;
//   t4.1: int = more t2.1 t3.1;
//   br t4.1 .branch1 .branch2;
// .branch1:
//   t5.1: int = const 1;
//   a.2: int = id t5.1;
//   a.1: int = id a.2;
//   jmp .branch0;
// .branch2:
//   t7.0: int = const 0;
//   print t7.0;
//   ret;
// }