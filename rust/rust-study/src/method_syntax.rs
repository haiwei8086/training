
pub fn run(){

    let a = Circle { x: 0.0, y: 0.0, radius: 2.0};
    a.reference();
    let mut b = Circle{ x: 0.0, y: 0.0, radius: 2.0};
    b.mutable_reference();
    let c = Circle{ x: 0.0, y: 0.0, radius: 2.0};
    c.takes_ownership();

    let d = Circle { x: 0.0, y: 0.0, radius: 2.0};
    println!("{}", d.area());

    let e = d.grow(3.0).area();
    println!("{}", e);

    let f = Circle::new(0.0, 0.0, 2.0);
    println!("Static method: {}", f.radius);

    println!("使用生成器模式");

    let g = CircleBuilder::new().x(1.0).y(1.0).radius(2.0).finalize();
    println!("area: {}", g.area());
}


struct Circle{
    x: f64,
    y: f64,
    radius: f64
}

impl Circle{

    // static methods
    fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle {x: x, y: y, radius: radius}
    }


    /*
        三种借用方式
        1. &self: 引用
        2. &mut self: 可变引用
        3. self: 获取 ownership
    */
    fn reference(&self){
        println!("引用: {}", self.radius);
    }

    fn mutable_reference(&mut self){
        self.radius = 3.0;
        println!("mutable引用: {}", self.radius);
    }

    fn takes_ownership(self){
        //self.radius = 4.0;
        println!("Borrowing ownership: {:?}", self.radius);
    }

    fn area(&self) -> f64 {
        3.1 * (self.radius * self.radius)
    }

    fn grow(&self, increment: f64) -> Circle {
        Circle { x: self.x, y: self.y, radius: self.radius + increment}
    }

}

// 生成器 模式
struct CircleBuilder {
    x: f64,
    y: f64,
    radius: f64
}

impl CircleBuilder {

    fn new() -> CircleBuilder {
        CircleBuilder {x: 0.0, y: 0.0, radius: 1.0}
    }

    fn x(&mut self, coordinate: f64) -> &mut CircleBuilder {
        self.x = coordinate;
        self
    }

    fn y(&mut self, coordinate: f64) ->&mut CircleBuilder {
        self.y = coordinate;
        self
    }

    fn radius(&mut self, coordinate: f64) -> &mut CircleBuilder {
        self.radius = coordinate;
        self
    }

    fn finalize(&self) -> Circle {
        Circle {x: self.x, y: self.y, radius: self.radius}
    }
}
