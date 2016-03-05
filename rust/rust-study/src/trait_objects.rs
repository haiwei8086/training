/*
    静态调度和动态调度
    动态调度会有些许性能损耗，但却可以有效的抑制内连(inline)和编译器相关优化机制
*/

pub fn run(){
    println!("Trait objects");

    let a = 5u8;
    let b = "Hello".to_string();

    println!("Dynamic dispatch: {}", dynamic_func(&a));
    println!("Dynamic dispatch: {}", dynamic_func(&b));

    println!("Static dispatch: {}", static_func(a));
    println!("Static dispatch: {}", static_func(b));
}

trait Foo { fn method(&self) -> String; }
impl Foo for u8 {
    fn method(&self) -> String { format!("u8: {}", *self) }
}

impl Foo for String {
    fn method(&self) -> String { format!("string: {}", *self) }
}

// Static dispatch
fn static_func<T: Foo>(x: T) -> String {
    x.method()
}

// Dynamic dispatch
fn dynamic_func(x: &Foo) -> String {
    x.method()
}
