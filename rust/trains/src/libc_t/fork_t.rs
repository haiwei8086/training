extern crate libc;

pub fn run(){
    println!("Libc proccess fork");

    for _ in 0..2 {
        let pid = unsafe { libc::fork() };

        if pid == 0 {
            worker();
            break;
        }else{
            println!("master: pid: {}, ppid: {}", unsafe { libc::getpid() }, unsafe { libc::getppid() });
        }
    }
}


fn worker(){
    println!("worker: pid: {}, ppid: {}", unsafe { libc::getpid() }, unsafe { libc::getppid() });
}
