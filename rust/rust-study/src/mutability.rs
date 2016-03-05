use std::sync::Arc;
use std::cell::RefCell;
use std::cell::Cell;


pub fn run(){

    let mut a = 5;
    let b = &mut a;
    println!("Binding to immutable: {}", b);
    *b = 6;
    println!("Change immutable: {}", b);

    let mut c = 1;
    let mut d = &mut c;
    println!("Binding to mutable: {}", d);
    *d = 2;
    println!("Change mutable: {}", d);


    let e = Arc::new(5);
    let f = e.clone();  // clone方法返回的是 &T 引用
    let i = e.clone();  // 同一时间可以存在多个immutable reference(不可变引用)
    println!("Clone method");

    let g = RefCell::new(42);
    let h = g.borrow_mut();  // borrow_mut方法返回的是 &mut T引用
    // let i = g.borrow_mut();  // 统一时间只能有一个&mut T(可变引用)
    println!("borrow mut method");

    file_leve_mutability();
}

// 你不能有一个struct里的一些字段是可变(mut), 另一些是不可变
/*
struct Point {
    x: i32,
    mut y: i32  // nope
}
*/
struct Point {
    x: i32,
    y: i32
}

// std::cell::Cell<T>
struct CellPoint{
    x: i32,
    y: Cell<i32>
}

fn file_leve_mutability(){
    
    let mut a = Point {x: 5, y: 6};
    a.x = 10;
    println!("Mutable struct: {}", a.x);


    let b = Point {x: 5, y: 6};
    // b.x = 10; // error: cannot assign to immutable field `b.x`
    println!("immutable struct: {}", b.x);


    let c = CellPoint{ x:5, y: Cell::new(6)};
    c.y.set(7);
    println!("Cell mutable: {:?}", c.y);
}
