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
struct Context {
    pub title: String,
    pub name: String,
}

impl Context {
    pub fn new() -> Self {
        Context {
            title: "Title".to_owned(),
            name: "Name".to_owned(),
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        println!("Drop Context, naem is {}", self.name);
    }
}



fn main() {
    println!("Size test");

    println!("Bool size: {}", mem::size_of::<bool>());

    println!("i32:123 size: {}", mem::size_of::<i32>());

    println!("u16:10 size: {}", mem::size_of::<[u16; 10]>());

    let str: &str = "master";
    println!("&str:master size: {}", mem::size_of_val(&str));

    /*
    let ctx_1 = Context::new();
    let ctx_2 = Context::new();

    println!("ctx_1: {:p}", &ctx_1 as *const _);
    println!("ctx_2: {:p}", &ctx_2 as *const _);


    let mut ctx_list: Vec<Context> = Vec::new();
    
    for i in 0..2 {
        println!("cyc time: {}", i);

        ctx_list.push(Context::new());
        println!("cyc: {}, c: {:p}", i, &ctx_list[i] as *const _);

        ctx_test(&mut ctx_list);
    }

    let len = ctx_list.len();

    ctx_list[len - 1].name = "changed name".to_owned();

    for i in 0..ctx_list.len() {
        println!("ctx_list cyc: {}, item: {:p}, name: {}", i, &ctx_list[i] as *const _, &ctx_list[i].name);
    }
    */

    let ptr = into();
    out(ptr);

}


fn ctx_test(ctx_list: &mut Vec<Context>) {
    let mut ctx2 = Context::new();
    println!("ctx_test:ctx2: {:p}", &ctx2 as *const _);
    ctx2.name = "Test name".to_owned();

    ctx_list.push(ctx2);
}


fn into() -> *mut usize {
    let c = Context::new();
    let box_c = Box::new(c);
    let c_ptr = Box::into_raw(box_c);

    println!("Ctx ptr: {:p}", c_ptr);

    c_ptr as *mut _
}

fn out(ptr: *mut usize) {
    let mut box_ctx: Box<Context> = unsafe { Box::from_raw(ptr as *mut Context) };
    box_ctx.name = "from ctx".to_owned();

    println!("Ctx ptr: {:p}", &(*box_ctx));
}