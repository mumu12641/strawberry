// this is a comment.

class Main inherits A{
    b : Int = 0;
    str: String = "str_test";

    function main(ii:Int) -> Int {
        let s:String = "hello";
        let t:Int = 0;
        t = 1;
        t.print_str(s);
        self.print_int(1);
        print_int(1 + 0);
        if(flag) then {
            print_int(1);
            print_int(1);
        }else{
            print_int(0);
        }
        while(true){
            print_int(1);
        }
        return new Int;
    };

    function funa(a:Int) -> Int{
        
    };
};

class A {
    a:Int = 0;
    fun funa(a:Int) -> Int{
        return 0;
    };
};
