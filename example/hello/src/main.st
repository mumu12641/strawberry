class Main {
	c:A;
	fun main() -> Int { 
		let a:A = new A;
		print(a.test());
		return 3 * 3; 
	};

};


/*
let a = false;
        if(!a) then {
            print("a = false\\n");
        }else{
            print("a = true");
        }
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