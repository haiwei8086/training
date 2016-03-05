/*
    traits 类似mixin
*/

pub fn run(){
    println!("traits run");

    println!("Print point info");
    let p1 = Point{x: 1, y:2};
    p1.print_info();

    let nv = NumberValid;
    nv.is_valid();
    nv.is_invalid();

    let sv = StringValid;
    sv.is_valid();
    sv.is_invalid();

    println!("Print i32 convert to i64: {:?}", normal(&1));

}


struct Point {
    x: i32,
    y: i32
}

trait PrintInfo {
    fn print_info(&self);
}

impl PrintInfo for Point {
    fn print_info(&self){
        println!("Point info: x: {}, y: {}", self.x, self.y);
    }
}

// Mixin
trait Mixin {
    fn is_valid(&self) -> bool;
    fn is_invalid(&self) -> bool {
        println!("Mixin is invalid: {:?}", !self.is_valid());
        !self.is_valid()
    }
}

struct NumberValid;
struct StringValid;

impl Mixin for NumberValid {
    fn is_valid(&self) -> bool {
        println!("Number valid alway true");
        true
    }
}

impl Mixin for StringValid {
    fn is_valid(&self) -> bool {
        println!("String valid alway false");
        false
    }

    fn is_invalid(&self) -> bool {
        println!("String invalid alway false");
        false
    }
}

trait ConvertTo<GenericType> {
    fn convert(&self) -> GenericType;
}
impl ConvertTo<i64> for i32 {
    fn convert(&self) -> i64{
        *self as i64
    }
}

fn normal<T: ConvertTo<i64>>(x: &T) -> i64 {
    x.convert()
}
