/*
    感觉 Option 有点像 es6 的 generator, 同样可以使用递归串连在一起，类似 co 的实现？

    enum Option<T> {
        None,
        Some(T),
    }

    处理 2 种结果的方式，一种 Ok, 一种 Err
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
*/


use std::result;
use std::io;
use std::num;
use std::num::ParseIntError;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::error::Error;


pub fn run(){

    /*
    let file_name = "error_handler.rs";
    match find_char_offset_in_str(file_name, '.') {
        None => println!("No file extension found!"),
        Some(i) => println!("File extension: {}", &file_name[i + 1..])
    }
    */

    println!("File extension: {}", extension("foobar.csv").unwrap_or("rs"));
    println!("File extension: {}", extension("foobar").unwrap_or("rs"));

    println!("For result type ----------------------------");

    match double_number_for_string("10") {
        Ok(n) => println!("Double number: {}", n),
        Err(err) => println!("Error: {:?}", err)
    }

    match double_number_for_string_alias("no parse") {
        Ok(n) => println!("Double number; {}", n),
        Err(err) => println!("Error: {:?}", err)
    }


    match double_number_option_result(extension("abc.2")){
        Ok(n) => println!("double_number_option_result: {}", n),
        Err(err) => println!("double_number_option_result Err: {:?}", err)
    }

    match double_number_option_result(extension("abc")){
        Ok(n) => println!("double_number_option_result: {}", n),
        Err(err) => println!("double_number_option_result Err: {:?}", err)
    }

    match double_number_option_result(extension("abc.rs")){
        Ok(n) => println!("double_number_option_result: {}", n),
        Err(err) => println!("double_number_option_result Err: {:?}", err)
    }

    let file_path = "/Volumes/mac-ext/study-sample/rust/rust-study/file_test.txt";

    match file_double(file_path) {
        Ok(n) => println!("file_double: {}", n),
        Err(err) => println!("file_double Error: {}", err)
    }

    match file_double_early(file_path) {
        Ok(n) => println!("file_double_early: {}", n),
        Err(err) => println!("file_double_early Error: {}", err)
    }

    match file_double_cli_error(file_path) {
        Ok(n) => println!("file_double_try: {}", n),
        Err(err) => println!("file_double_try Error: {:?}", err)
    }

}


/// find char in string, return offset used Option
///
/// # Examples
/// ```
/// let file_name = "error_handler.rs";
/// match find_char_offset_in_str(file_name, '.') {
///     None => println!("No file extension found."),
///     Some(i) => println!("File extension: {}", &file_name[i+1..]),
///  }
/// ```
fn find_char_offset_in_str(text_str: &str, needle: char) -> Option<usize> {
    for (offset, c) in text_str.char_indices(){
        if c == needle {
            return Some(offset);
        }
    }
    return None;
}


/// find extension of a file
/// standard library: https://doc.rust-lang.org/stable/std/path/struct.Path.html#method.extension
/// # Examples
///```
/// assert_eq!(extension("foobar.csv").unwrap_or("rs"), "csv");
/// assert_eq!(extension("foobar").unwrap_or("rs"), "rs");
///```
fn extension(file_name: &str) -> Option<&str> {
    /*
    fn map<F, T, A>(option: Option<T>, f: F) -> Option<A> where F: FnOnce(T) -> A {
        match option {
            None => None,
            Some(value) => Some(f(value)),
        }
    }
    */


    return find_char_offset_in_str(file_name, '.').map(|i| &file_name[i + 1..]);
}

// 串联使用
// standard library: https://doc.rust-lang.org/stable/std/option/enum.Option.html#method.unwrap_or
fn unwrap_or<T>(option: Option<T>, default: T) -> T {
    match option {
        None => default,
        Some(value) => value
    }
}

/*
fn and_then<F, T, A>(option: Option<T>, f: F) -> Option<A>
    where F: FnOnce(T) -> Option<A> {
    match option {
        None => None,
        Some(value) => f(value),
    }
}
*/


fn double_number_for_string(number_str: &str) -> Result<i32, ParseIntError>{
    match number_str.parse::<i32>() {
        Ok(n) => Ok(2 * n),
        Err(err) => Err(err)
    }

    // or number_str.parse::<i32>().map(|n| 2 * n)
}

/*
fn double_number_for_string_option(number_str: &str) -> Option<i32>{
    // 如果返回的是 Option，就可以在此统一做错误处理
}
*/


/*
    定义 result 别名，就像在标准库中常见的 Result<I32>,
    因为第二个参数永远是固定的 ParseIntError
*/
type ResultAlias<T> = result::Result<T, ParseIntError>;

fn double_number_for_string_alias(number_str: &str) -> ResultAlias<i32>{
    number_str.parse::<i32>().map(|n| 2 * n)
}


/*
    Option and Result 同时处理, 此类情况就是将 Option 转换成 Result
    ok_or 处理 Option None
    and_then 串连转换操作，同时处理Error返回给 Result
*/
fn double_number_option_result(option: Option<&str>) -> Result<i32, String>{
    option
        .ok_or("Please give the value!".to_owned())
        .and_then(|v| v.parse::<i32>().map_err(|err| err.to_string()))
}


fn file_double<P: AsRef<Path>>(file_path: P) -> Result<i32, String>{
    File::open(file_path)
        .map_err(|err| err.to_string())
        .and_then(|mut file| {
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .map_err(|err| err.to_string())
                .map(|_| contents)
        })
        .and_then(|contents| {
            contents.trim().parse::<i32>()
                .map_err(|err| err.to_string())
        })
        .map(|n| 2 * n)
}


fn file_double_early<P: AsRef<Path>>(file_path: P) -> Result<i32, String>{

    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(err) => return Err(err.to_string())
    };

    let mut contents = String::new();
    if let Err(err) = file.read_to_string(&mut contents){
        return Err(err.to_string());
    }

    let n: i32 = match contents.trim().parse() {
        Ok(n) => n,
        Err(err) => return Err(err.to_string())
    };

    Ok(2 * n)
}

/*
    自定义 Error type
*/
#[derive(Debug)]
enum CliError {
    Io(io::Error),
    ParseNumber(num::ParseIntError)
}


impl From<io::Error> for CliError{
    fn from(err: io::Error) -> CliError{
        CliError::Io(err)
    }
}

impl From<num::ParseIntError> for CliError {
    fn from(err: num::ParseIntError) -> CliError{
        CliError::ParseNumber(err)
    }
}


fn file_double_try<P: AsRef<Path>>(file_path: P) -> Result<i32, CliError>{
    let mut file = try!(File::open(file_path).map_err(CliError::Io));

    let mut contents = String::new();
    try!(file.read_to_string(&mut contents).map_err(CliError::Io));

    let n: i32 = try!(contents.trim().parse().map_err(CliError::ParseNumber));

    Ok(2 * n)
}

/*
macro_rules! try {
    ($e:expr) => (match $e {
        Ok(val) => val,
        Err(err) => return Err(::std::convert::From::from(err)),
    });
}
*/
// 通用型 Box<Error>
fn file_double_box_error<P: AsRef<Path>>(file_path: P) ->Result<i32, Box<Error>>{

    let mut file = try!(File::open(file_path));
    let mut contents = String::new();
    try!(file.read_to_string(&mut contents));

    let n = try!(contents.trim().parse::<i32>());

    Ok(2 * n)
}


// 自定义转通用型
fn file_double_cli_error<P: AsRef<Path>>(file_path: P) ->Result<i32, CliError>{

    let mut file = try!(File::open(file_path));
    let mut contents = String::new();
    try!(file.read_to_string(&mut contents));

    let n = try!(contents.trim().parse::<i32>());

    Ok(2 * n)
}
