class Main { 
	a:Int = 0;
	fun main() -> Int { 
		let a = false;
        if(!a) then {
            print("a = false \\n");
        }else{
            print("a = true \\n");
        }
		return fib(10);
	};

	fun a_funtion() -> Int {
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

