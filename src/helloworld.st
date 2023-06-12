class Main { 
	a:Int = 0;
	fun main() -> Int { 
		let a = new A;
		a_funtion();
		let a = false;
        if(!a) then {
            print("a = false \\n ");
        }else{
            print("a = true \\n");
        }
		return 0;
	};

	fun a_funtion() -> Int {
		// print(a.geta());
		return 0;
	};
};

class A{
    fun geta() -> String {
        return "this is A\\n";
    };
};