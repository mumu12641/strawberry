class Main {
    fun main() -> Int{
        let flag :Int = 1;
        let i : Int = 30;
        
        while(i > 0){
            print("i > 0\\n");
            i = i - 1;
        }
        
        if (flag > 0) then {
            print("flag > 0\\n");
        }else {
           //print("flag < 0\\n");
            
            if (flag == 0) then{
                print("flag == 0\\n");
            }else {
                print("flag < 0\\n");
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