class Main { 
	a:Int = 0;
	fun main() -> Int { 
		let a = new A;
		a.print("Hello world!!!! \\n"); 
		// a.print(a.geta());
		a_funtion();
		return fib(10);
	};

	fun a_funtion() -> Int {
	 // a.print(a.geta());
		return 0;
	};

	fun fib(n:Int) -> Int{
		if (n<=1) then {
			return n;
		}else{
			return fib(n - 1) + fib(n - 2);
		}
	};
};

