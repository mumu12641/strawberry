class Main {
    fun main() -> Int{
        let a:B = get();
        print(a.getb());
        return 0;
    };

    fun get() -> B{
        return new B;
    };

};

class A{
    fun geta() -> String {
        return "this is A\\n";
    };
};

class B inherits A{
    fun getb() -> String {
        return "this is B\\n";
    };
};