use std::{mem};
use winapi::shared::ws2def::{WSABUF};


const MAX_BUFFER_LEN: u32 = 4096;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_in {
    pub sin_family: i32,
    pub sin_port: u16,
    pub sin_addr: in_addr,
    pub sin_zero: [i8; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct in_addr {
    pub s_addr: u32,
}

#[derive(Clone)]
struct PerIOContext {
    pub wsa_buf: WSABUF,                    // 存储数据的缓冲区，用来给重叠操作传递参数的，关于WSABUF后面
    pub buf: [i8; MAX_BUFFER_LEN as usize], // 真正接收数据得buffer
}

struct Context2 {
    pub title: String,
    pub name: String,
}

impl PerIOContext {
    pub fn new() -> Self {
        let mut ctx = PerIOContext {
            wsa_buf: unsafe { mem::zeroed() },
            buf: unsafe { std::mem::uninitialized() },
        };

        ctx.wsa_buf = WSABUF {
            len: MAX_BUFFER_LEN,
            buf: ctx.buf.as_ptr() as *mut _,
        };

        ctx
    }
}


impl Context2 {
    pub fn new() -> Self {
        Context2 {
            title: "Title".to_owned(),
            name: "Name".to_owned(),
        }
    }
}


fn main() {
    println!("Size test");

    println!("Bool size: {}", mem::size_of::<bool>());

    println!("i32:123 size: {}", mem::size_of::<i32>());

    println!("u16:10 size: {}", mem::size_of::<[u16; 10]>());

    let str: &str = "master";
    println!("&str:master size: {}", mem::size_of_val(&str));

    
    let ctx_1 = PerIOContext::new();
    let ctx_2 = PerIOContext::new();

    println!("ctx_1: {:p}", &ctx_1 as *const _);
    println!("ctx_2: {:p}", &ctx_2 as *const _);

    let mut ctx_list = vec![Box::new(PerIOContext::new()); 2];

    for i in 0..2 {
        println!("cyc time: {}", i);

        let c = PerIOContext::new();
        println!("cyc: {}, c: {:p}", i, &c as *const _);

        ctx_list[i] = Box::new(c);
        println!("cyc: {}, ctx_list: {:p}", i, &ctx_list[i] as *const _);

        ctx_test();
    }

    for i in 0..2 {
        println!("ctx_list cyc: {}, item: {:p}", i, &ctx_list[i] as *const _);
    }
}


fn ctx_test() {
    let ctx = PerIOContext::new();
    println!("ctx_test:ctx: {:p}", &ctx as *const _);

    let ctx2 = Context2::new();
    println!("ctx_test:ctx2: {:p}", &ctx2 as *const _);


    let box_ctx = Box::new(PerIOContext::new());
    println!("ctx_test:box_ctx: {:p}", &box_ctx as *const _);
    

    let addr: sockaddr_in = unsafe { mem::zeroed() };
    println!("ctx_test:addr: {:p}", &addr as *const _);
}