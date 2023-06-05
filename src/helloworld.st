class Main {
    fun main() -> Int{
        let flag :Int = -2;
        let i : Int = 30;
        
        while(i > 0){
            print("i > 0\\n");
            i = i - 1;
        }
        
        if (flag  > -1) then {
            print("flag > -1\\n");
        }else {
            if (flag == -1) then{
                print("flag == -1\\n");
            }else {
                print("flag < -1\\n");
            }
            
        }
        
        return 0;
    };

};

class A{
    fun get() -> String {
        return "A.str\\n";
    };
};