
/*
小结

ownership
在变量相互做赋值操作的时候会涉及到 move 和 copy 操作，
rust的基本类型(i32, bool...)都实现了 Copy，所以赋值是copy操作，
而Vec和struct在赋值操作时是 move操作，仅仅是将heap指针交给新的变量，
如 let x = vec![]; let y = x; 当交给 y 指针后，x将不能在使用，
因为编译器会根据情况，可能将 x 从内存中移除。

borrow
指针 move 导致的问题，将由borrow解决。
一般借有2中方法：
1. &T, 不可变(immutable)借用 ownership
2. &mut T, 可变(mutable)借用 ownership

borrow的规则：
1. 任何borrow 不得大于 owner scope
2. 为了防止数据竞争，同一时间只能有一个 &mut T, 可有有多个 &T
   可以使用｛｝块隔离出一个小的scope，保证外层和内层scope中始终只有一个 &mut T

lefetime
用来防止使用完成后自动释放(use after free)，
lefttime有显示和隐式2中声明方法，分别用在声明：静态变量，函数，struct,impl等
1.隐式：fn f(i: &32) {}
2.显示：fn f<'a> (i: &'a 32) {}

在隐式使用时，lefetime的规则：
1.Each elided lifetime in a function’s arguments becomes a distinct lifetime parameter.

fn print(s: &str); // elided
fn print<'a>(s: &'a str); // expanded

2.If there is exactly one input lifetime, elided or not, that lifetime is assigned
  to all elided lifetimes in the return values of that function.

  fn frob(s: &str, t: &str) -> &str; // ILLEGAL, two inputs
  fn frob<'a, 'b>(s: &'a str, t: &'b str) -> &str; // Expanded: Output lifetime is ambiguous

3.If there are multiple input lifetimes, but one of them is &self or &mut self,
  the lifetime of self is assigned to all elided output lifetimes.

fn args<T:ToCStr>(&mut self, args: &[T]) -> &mut Command // elided
fn args<'a, 'b, T:ToCStr>(&'a mut self, args: &'b [T]) -> &'a mut Command // expanded

特殊情况lefetime

fn foo<'a>() -> &'a str
fn get_str() -> &str; // ILLEGAL, no inputs
*/


pub fn run(){

    //complie_error();
    //copy_types();
    borrowing_ownership_sample();
    borrowing_mutable_ownership_sample();
    lifetime_for_struct();
}


fn move_semantics(){

    /*
    在stack创建一个Vec指针，指向heap的一个存储块，
    在里面存放着 1,2,3.

    声明一个新的变量v2，将v存放的指针move到v2
    这里是move不是copy，如果是copy将会有2个指针指向heap存储块
    这将违反rust的防止数据竞争的安全规则，
    所以不能在moved后再次使用该变量v
    */
    let v = vec![1, 2, 3];
    let v2 = v;

    // 一下都会触发编译错误
    // error: use of moved value: `v`
    // println!("{}", v[0]);
    // move_print(v);
    // v = back_ownership_bad(v); error: cannot borrow immutable borrowed content `*v` as mutable
}

fn move_print(v: Vec<i32>){}

fn back_ownership_bad(v: Vec<i32>) -> Vec<i32> {
    // do stuff with v
    // hand back ownership
    v
}


fn copy_types(){

    /*
    所有的基础类型都实现了copy特性
    */

    let a = 5;
    let _y = copy_double(a);
    println!("{}, {}", a, _y);
}

fn copy_double(v: i32) -> i32 {
    v * 2
}

/*
Borrowing 规则
1.所有的借用都必须维持在一个scope,并且不能比owner更大的scope
2.同一时间可以有多个&T借用，但是只能有一个&mut T借用(可以有多个读，但是只能有一个写)
*/
fn borrowing_ownership_sample(){
    /*
    &T type是一个引用，而非拥有资源，只是借用ownership
    由于绑定的资源不释放，所以下面在方法调用之后，仍然可是使用
    */
    let mut v1 = vec![1, 2, 3];
    let v2 = vec![1, 2, 3];

    let answer = borrowing_ownership_fn(&v1, &v2);

    println!("{:?}, {}", v1, answer);
}

fn borrowing_ownership_fn(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
    // do stuff with v1 and v2
    // return the answer
    42
}

fn borrowing_mutable_ownership_sample(){
    /*
    error: cannot borrow `x` as immutable because it is also borrowed as mutable, println!("{}", x);

    let mut x = 5;
    let y = &mut x;    // -+ &mut borrow of x starts here
                       //  |
    *y += 1;           //  |
                       //  |
    println!("{}", x); // -+ - try to borrow x here
                       // -+ &mut borrow of x ends here
   */

   /*
   鉴于Borrowing的规则，同一时间只能有一个&mut T, 所以我们要增加｛｝
   不能让 &x 和 y在同一范围,
   *y (星号)指定的是内存位置的数据，而不是&mut T指针
   */
    let mut x = 5;      // 变量本身正在占用 &mut T 引用
    {
        let y = &mut x; // -+ &mut borrow 准备借用
        *y += 1;        //  |
    }                   // -+ ... 结束借用

    println!("{}", x);  // <- try to borrow x here


    // 下面展示不变(immutable)借用一个可变Vec(mutable Vec)
    // immutable borrow可以防止借用期间串改
    let mut v = vec![1, 2, 3];
    for i in &v {
        println!("{}", i);
    }

}

// 隐视
fn lifetime_implicit(x: &i32) {}
// 显示lifetime
fn lifetime_explicit<'a>(x: &'a i32) {}
fn lifetime_explicit_two<'a, 'b>(x: &'a i32, y: &'b i32) {}
// 2个参数和返回值需要相同的scope
fn lifetime_explicit_multiple<'a>(x: &'a i32, y: &'a i32) -> &'a i32 { x }


struct Foo {
    x: i32
}

//为什么需要这个，我们需要确保所有对struct的引用不能超出i32的lifetime
struct LifetimeFoo<'a> {
    x: &'a i32
}

// 实现需要一个'a保证，同时LifetimeFoo本身也需要一个'a保证lifetime
impl<'a> LifetimeFoo<'a> {
    fn x(&self) -> &'a i32 { self.x }
}


fn lifetime_for_struct(){

    let a = 5;
    let f = Foo {x: a};
    println!("lifttime_for_struct: {}", f.x);

    let y = &6;  // this is the same as `let _y = 5; let y = &_y;`
    let lf = LifetimeFoo {x: y};

    println!("lifttime_for_struct: {}", lf.x);
    println!("lifttime_for_struct: {}", lf.x());

    // static lifetime 是在整个程序的生命周期的,不过只在当前scope
    let static_str: &'static str = "Hello, world.";
    static FOO: i32 = 5;
    let static_x: &'static i32 = &FOO;

    println!("{}", static_str);
}


fn lifetime_sample(){
    /*
    有些情况是不能省略lifetime的，如下面集中情况

    fn foo<'a>() -> &'a str
    fn get_str() -> &str; // ILLEGAL, no inputs
    fn frob(s: &str, t: &str) -> &str; // ILLEGAL, two inputs
    fn frob<'a, 'b>(s: &'a str, t: &'b str) -> &str; // Expanded: Output lifetime is ambiguous
    */

}
