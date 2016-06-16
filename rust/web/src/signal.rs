extern crate libc;


use std::{mem, ptr, thread, time};


extern "C" {
    fn sigprocmask(signum: libc::c_int, set: *const libc::sigset_t, oldset: *const libc::sigset_t) -> libc::c_int;
    fn sigsuspend(set: *mut libc::sigset_t) -> libc::c_int;
}

#[derive(Copy, Clone)]
struct Sigaction {
    pub signo: libc::c_int,
    pub name: &'static str,
    pub handler: libc::sighandler_t,
}

const SIGNALS : [Sigaction; 10] = [
    Sigaction {
        signo: libc::SIGHUP,
        name: "SIGHUP",
        handler: 0,
    },
    Sigaction {
        signo: libc::SIGQUIT,
        name: "SIGQUIT",
        handler: 0,
    },
    Sigaction {
        signo: libc::SIGTERM,
        name: "SIGTERM",
        handler: 0,
    },
    Sigaction {
        signo: libc::SIGWINCH,
        name: "SIGWINCH",
        handler: 0,
    },
    Sigaction {
        signo: libc::SIGALRM,
        name: "SIGALRM",
        handler: 0,
    },
    Sigaction {
        signo: libc::SIGINT,
        name: "SIGINT",
        handler: 0,
    },
    Sigaction {
        signo: libc::SIGIO,
        name: "SIGIO",
        handler: 0,
    },
    Sigaction {
        signo: libc::SIGCHLD,
        name: "SIGCHLD",
        handler: 0,
    },
    Sigaction {
        signo: libc::SIGSYS,
        name: "SIGSYS",
        handler: libc::SIG_IGN,
    },
    Sigaction {
        signo: libc::SIGPIPE,
        name: "SIGPIPE",
        handler: libc::SIG_IGN,
    }
];


pub fn run()
{
    init_signal();

    let mut sigset: libc::sigset_t = unsafe { mem::uninitialized() };
    unsafe { libc::sigemptyset(&mut sigset as *mut libc::sigset_t) };

    unsafe { libc::sigaddset(&mut sigset as *mut libc::sigset_t, libc::SIGCHLD) };
    unsafe { libc::sigaddset(&mut sigset as *mut libc::sigset_t, libc::SIGALRM) };
    unsafe { libc::sigaddset(&mut sigset as *mut libc::sigset_t, libc::SIGIO) };
    unsafe { libc::sigaddset(&mut sigset as *mut libc::sigset_t, libc::SIGINT) };
    unsafe { libc::sigaddset(&mut sigset as *mut libc::sigset_t, libc::SIGHUP) };
    unsafe { libc::sigaddset(&mut sigset as *mut libc::sigset_t, libc::SIGUSR1) };
    unsafe { libc::sigaddset(&mut sigset as *mut libc::sigset_t, libc::SIGWINCH) };
    unsafe { libc::sigaddset(&mut sigset as *mut libc::sigset_t, libc::SIGTERM) };
    unsafe { libc::sigaddset(&mut sigset as *mut libc::sigset_t, libc::SIGQUIT) };
    unsafe { libc::sigaddset(&mut sigset as *mut libc::sigset_t, libc::SIGUSR2) };

    if -1 == unsafe { sigprocmask(libc::SIG_BLOCK, &sigset as *const libc::sigset_t, ptr::null()) }
    {
        println!("Sigprocmask failed!");
    } else {
        println!("Sigprocmask successed.");
    }

    unsafe { libc::sigemptyset(&mut sigset as *mut libc::sigset_t) };

    for _ in 0..2 {
        let pid = unsafe { libc::fork() };

        if pid == 0 {
            worker();
            return;
        } else {
            println!("Master");
        }
    }

    println!("Proccess signal suspend");

    loop {
        unsafe { sigsuspend(&mut sigset as *mut libc::sigset_t) };

        println!("Proccess received a signal and exit");

        break;
    }
}

pub fn init_signal()
{
    for sig in SIGNALS.iter()
    {
        let mut action = unsafe { mem::uninitialized::<libc::sigaction>() };
        let _ = unsafe { libc::sigemptyset(&mut action.sa_mask as *mut libc::sigset_t) };
        if sig.handler != libc::SIG_IGN
        {
            action.sa_sigaction = signal_handler as usize;
        }

        if -1 == unsafe { libc::sigaction(sig.signo, &action as *const libc::sigaction, ptr::null_mut()) }
        {
            println!("Register sigaction failed!");
        } else {
            println!("Register sigaction: {:?}", sig.name);
        }
    }
}

fn worker()
{
    let pid = unsafe { libc::getpid() };
    println!("In workder... PID: {:?}", pid);

    thread::sleep(time::Duration::from_millis(2000));

    unsafe { libc::kill(pid, libc::SIGQUIT) };

    println!("Kill workder... PID: {:?}", pid);
}



pub fn signal_handler(signo: usize)
{
    println!("In signal handler: {:?}", signo);
}
