use std::thread;
use std::sync::{Arc, Mutex, mpsc};

pub fn run(){
    wait_thread_done();
    concurrency_for_channel();
}


fn wait_thread_done(){
    println!("等待线程完成");

    let handle = thread::spawn(|| {
        "由线程返回的值"
    });

    println!("{}", handle.join().unwrap());
}


fn concurrency_for_channel(){
    println!("channel实现多线程并行");

    let data = Arc::new(Mutex::new(0));

    let (tx, rx) = mpsc::channel();

    for _ in 0..10{
        let (data, tx) = (data.clone(), tx.clone());

        thread::spawn(move || {
            let mut data = data.lock().unwrap();

            *data += 1;

            tx.send(());
        });
    }

    for i in 0..10{
        println!("Wait recv: {}", i);
        rx.recv().ok().expect("No receive data");
    }
}
