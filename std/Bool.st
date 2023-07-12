/**
* This is the definition of the built-in object in strawberry, implemented using inline assembly, please do not modify!!
* @author: https://github.com/mumu12641
* @update: 2023/7/12
*/

class Bool inherits Object {
    val : PrimSlot;
    public fn to_string() -> String {
        if (!self){
            return "false";
        }else{
            return "true";
        }
        return "";
    };
};