
pub fn run(){

    // variable bindings
    // let (x, y) = (1, 2);
    // x = y = 5

    /*
    let mut y = 5;
    let x = (y = 6);  // x has the value `()`, not `6`
    */

    // Function pointers
    let f: fn(i32) -> i32 = func_pointer;
    let j = func_pointer;
    let six = j(5);
    println!("{}", six);


}


fn format_print(){
    format!("Hello");
    format!("Hello, {}!", "world");
    format!("The number is {}", 1);
    format!("{:?}", (3, 4));
    format!("{value}", value=4);
    format!("{} {}", 1, 2);
}

/*
    A diverging function can be used as any type
    RUST_BACKTRACE=1 cargo run

    let x: i32 = diverges();
    let x: String = diverges();
*/
fn diverges() -> ! {
    panic!("This function never returns!");
}


fn func_pointer(i: i32) -> i32 {
    i + 1
}


fn basic_type(){

    // type bool
    let x = true;
    let y: bool = false;

    // type char
    let x = 'x';
    let two_hearts = '💕';

    // array
    let a = [0; 20]; // a: [i32; 20]
    println!("a has {} elements", a.len());

    // Slices
    let b = [0, 1, 2, 3, 4];
    let middle = &b[1..4]; // A slice of a: just the elements 1, 2, and 3
    let complete = &b[..]; // A slice containing all of the elements in a

    // Tuples
    let x = (1, "hello");
    let x: (i32, &str) = (1, "hello");
    (0,); // single-element tuple
    (0); // zero in parentheses
    let tuple = (1, 2, 3);
    let x = tuple.0;
    let y = tuple.1;
    let z = tuple.2;

}

fn flow_if(){
    let x = 5;

    if x == 5 {
        println!("x is five!");
    } else if x == 6 {
        println!("x is six!");
    } else {
        println!("x is not five or six :(");
    }

    let y = if x == 5 { 10 } else { 15 }; // y: i32
}

fn flow_loop(){
    /*
        无限循环
        loop {
            println!("Loop forever!");
            break;
        }
    */


    // while loop
    let mut x = 5; // mut x: i32
    let mut done = false; // mut done: bool

    while !done {
        x += x - 3;

        println!("{}", x);

        if x % 5 == 0 {
            done = true;
        }
    }

    for x in 0..10 {
        println!("{}", x); // x: i32
    }

    // Enumerate
    for (i,j) in (5..10).enumerate() {
        println!("i = {} and j = {}", i, j);
    }
    /*
        i = 0 and j = 5
        i = 1 and j = 6
        i = 2 and j = 7
        i = 3 and j = 8
        i = 4 and j = 9
    */
    // On iterators:
    for (linenumber, line) in (5..10).enumerate() {
        println!("{}: {}", linenumber, line);
    }


    'outer: for x in 0..10 {
        'inner: for y in 0..10 {
            if x % 2 == 0 { continue 'outer; } // continues the loop over x
            if y % 2 == 0 { continue 'inner; } // continues the loop over y
            println!("x: {}, y: {}", x, y);
        }
    }


}
