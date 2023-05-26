class Main inherits A{
    b:Int = 12;
    fun main() -> Int{
        // let a:Int = new Int;
        test(b);
        return 1;
    };

    fun test(a:Int) -> Int{
        return a;
    };
};
class A{
    // c:Int = 0;
};