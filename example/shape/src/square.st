class Square inherits Shape{

    a:Int = 0;

    fun draw() -> Int{
        name = "Rectangle \\n";
        print(name);
        return 0;
    };

    fun set(x:Int) -> Int{
        a = x;
        return 0;
    };

    fun get_a() -> Int{
        return a;
    };


    fun get_area() -> Int{
        return a*a;
    };

};