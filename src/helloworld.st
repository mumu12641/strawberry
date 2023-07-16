/*


*/
class Main{
	a:Int =1;
	fun main() -> Int{
		__asm__("movq 24(%rbp), %rax");
		return self.a;
	};
};