
pub fn run(){

    println!("Loop iterator");

    let mut range = 0..3;
    loop {
        match range.next() {
            Some(x) => {
                println!("{}", x);
            },
            None => { break }
        }
    }

    println!("迭代 vec");
    let nums = vec![11, 22, 33];

    println!("使用索引访问，不够直观");
    for i in 0..nums.len() {
        println!("{}", nums[i]);
    }


    println!("更直观的访问");
    for num in &nums {
        /*
            此处num是 &i32 不是 i32 本身
            下面为什么使用 ＊，而不是 &
            ＊ 将直接引用数据本身，而 & 会使用数据一个拷贝的副本
        */
        // num => &i32 not i32
        println!("{}", *num);
        //println!("{}", &num);
    }

    println!("Collect返回一个集合");
    let one_to_hundred = (1..100).collect::<Vec<i32>>();
    println!("{}", one_to_hundred.len());

    println!("Find iterator");
    let greater_than_forty_two = (0..100).find(|x| *x > 42);
    match greater_than_forty_two {
        Some(_) => println!("We got some numbers!"),
        None => println!("No numbers found :(")
    }

    println!("Fold iterator");
    let sum = (1..4).fold(0, |sum, x| sum + x);
    println!("{:?}", sum);

    println!("迭代器是惰性的，下面的代码不会创建1-99的数字，只是保存该序列的值");
    // let nums_range = 1..100;

    println!("简单迭代Vec");
    let numbers = vec![1, 2, 3];
    for num in numbers.iter(){
        println!("{}", num);
    }

    println!("惰性迭代器罢工, 可以使用for");
    (1..100).map(|x| println!("{:?}", x));

    println!("Take 迭代适配器");
    for i in (1..).take(3) {
        println!("{:?}", i);
    }

    println!("Filter 迭代适配器, filter 闭包返回true or false, 同时允许一个闭包参数");
    // filter不消费数据，在此可以直接饮用(&)对象本身
    for i in (1..10).filter(|&x| x % 2 == 0) {
        println!("{:?}", i);
    }

    println!("迭代器串联使用事例");
    for i in (1..)
            .filter(|&x| x % 2 == 0)
            .filter(|&x| x % 3 == 0)
            .take(5)
            .collect::<Vec<i32>>(){

        println!("{:?}", i);
    }

}
