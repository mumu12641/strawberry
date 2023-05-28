class Main inherits A{
    b:Int = 1024;
    fun main() -> Int{
        // let a:Int = new Int;
        test(b);
        return b;
    };

    fun test(a:Int) -> Int{
        return a;
    };
};
class A{
    // c:Int = 0;
};