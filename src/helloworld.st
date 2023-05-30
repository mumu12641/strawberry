class Main {
    a:String = "Main's a";
    fun main() -> Int{
        // let a : A = new A;
        let s : String= "hello world!!!! \\n";
        
        a.print(a);
        
//        print(s);
        return 0;
    };

    fun test(a:Int) -> Int{
        let a:Int = 5;
        return a;
    };
};

class A{

    fun get() -> String {
        return "A.str\\n";
    };
};