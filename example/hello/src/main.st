class Main {
	c:A;
	fun main() -> Int { 
		let a:A;
		if(null(a)) then {
			print("a is null");
		}else{
			print("a is not null \\t");
			print(a.test());
		}

		let b = false;
        if(null(test())) then {
            print("b = false\\n");
        }else{
            print("b = true");
        }

		return 3 * 3; 
	};
	fun test() -> Bool {
		return false;
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