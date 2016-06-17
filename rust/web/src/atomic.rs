extern crate libc;

use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time;


pub fn run()
{
    println!("Atomic operation.");

    let mutex_data = Arc::new(Mutex::new(0));
    let atomic_data = Arc::new(AtomicUsize::new(0));

    for i in 0..4 {
        let pid = unsafe { libc::fork() };

        let mutex_data = mutex_data.clone();
        let atomic_data = atomic_data.clone();

        if pid == 0 {
            println!("Child worker {} PID: {:?}", i, unsafe { libc::getpid() });

            // mutex_worker(i as usize, &mutex_data);
            atomic_workder(i as usize, &atomic_data);
        } else {
            return;
        }
    }
}


fn mutex_worker(n: usize, data: &Arc<Mutex<i32>>)
{
    println!("Mutex child {} PID: {:?}", n, unsafe { libc::getpid() });

    let mut d = data.lock().unwrap();
    *d += 1;
    println!("Mutex child {} Lock data: {:?}", n, *d);

    if n == 0 {
        thread::sleep(time::Duration::from_millis(3000));
    } else {
        thread::sleep(time::Duration::from_millis(1000));
    }

    println!("Mutex worker: {} end", n);
}

fn atomic_workder(n: usize, data: &Arc<AtomicUsize>)
{
    if n == 2 {
        return;
    }

    //data.store(n, Ordering::Relaxed);
    data.fetch_add(1, Ordering::Relaxed);
    println!("Atomic child {} lock: {:?}", n, data);

    println!("Atomic worker: {} end", n);

}
