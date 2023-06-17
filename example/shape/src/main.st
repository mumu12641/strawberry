class Main { 
	fun main() -> Int { 
		let rect:Rectangle = new Rectangle;

		let squ = new Square;

		let s = "asd";

		rect.set(3,4);
		squ.set(2);
		
		return rect.get_area() + squ.get_area();
		// return rect.get_area() + 2;
	};
};
