class Main inherits A{
	a:Int = 12;
	b:Int = 12;
	fun main() -> Int { 
		let a = new A;
		print(a.test());
		return b; 
	};

};


/*
		let a = false;
        if(!a) then {
            print("a = false\\n");
        }else{
            print("a = true");
        }
		
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
		*/