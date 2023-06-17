class Rectangle inherits Shape{

    a:Int = 0;
    b:Int = 0;

    fun draw() -> Int{
        name = "Rectangle \\n";
        print(name);
        return 0;
    };

    fun set(x:Int, y:Int) -> Int{
        a = x;
        b = y;
        return 0;
    };

    fun get_a() -> Int{
        return a;
    };

    fun get_b() -> Int{
        return b;
    };

    fun get_area() -> Int{
        return a*b;
    };

};