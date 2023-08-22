
class Main{
public fn main() -> int{
let a:int=0;
// let b:int = a + 1;
// if(a>1){
// a=1;
// }else{
// a=0;
// }
while(a >1){
a=1;
b=1;
}
return 0;
};
};
// 
// 
// 
// 
// @Main.main: int {
//   t1: int = const 0;
//   t2: int = const 1;
//   t3: Object = add t1 t2;
//   a = t3;
// .branch0:
//   t4 = a;
//   t5: int = const 1;
//   t6: int = const 3;
//   t7: Object = add t5 t6;
//   t8: Object = more t4 t7;
//   br t8 .branch1 .branch2;
// .branch1:
//   t9 = a;
//   t10: int = const 1;
//   t11: Object = add t9 t10;
//   a = t11;
//   jmp .branch0;
// .branch2:
//   t12: int = const 0;
//   ret t12
// }