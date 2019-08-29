#![allow(dead_code)]
#![allow(unused_imports)]


use std::time;
use std::thread::{spawn, sleep};
use std::sync::{mpsc, Arc, Mutex};



fn main() {
    
    //version_1();

    //version_2();

    version_3();
}



#[derive(Debug)]
struct Transaction {
    amount: isize,
    timestamp: u64,
    txid: String,
}
#[derive(Debug)]
struct Account {
    account_number: String,
    transactions: Mutex<Vec<Transaction>>,
    acct_type: String,
}

impl Account {
    pub fn new(number: &str) -> Self {
        Account {
            account_number: number.to_owned(),
            acct_type: String::new(),
            transactions: Mutex::new(Vec::new()),
        }
    }
}


#[derive(Debug)]
struct Account2 {
    account_number: String,
    transactions: Vec<Transaction>,
    acct_type: String,
}


#[derive(Debug)]
struct Context {
    pub stop: bool,
}



fn version_1() {
    let my_savings = Arc::new(Account::new("0001"));
    let feed_account = my_savings.clone(); // clones the ref, not the item
    let mobile_account = my_savings.clone();

    let file_feed = spawn(move || {

        let mut tx_guard = feed_account.transactions.lock().unwrap();

        tx_guard.push(Transaction {
            amount: 500,
            timestamp: 12,
            txid: "tx-001".to_owned(),
        });

        tx_guard.push( Transaction {
            amount: 750,
            timestamp: 4,
            txid: "tx-002".to_owned(),
        })
    });

    let mobile_feed = spawn(move || {

        mobile_account.transactions.lock().unwrap().push(Transaction {
            amount: 50,
            timestamp: 7,
            txid: "tx-003".to_owned(),
        });
    });

    file_feed.join().unwrap();
    mobile_feed.join().unwrap();

    println!("mutating from bg threads:\n\t{:?}", my_savings.transactions);
}


fn version_2() {
    let (tx, rx) = mpsc::channel();

    let tx2 = mpsc::Sender::clone(&tx);

    let file_feed2 = spawn(move || {
        tx.send(Transaction {
            amount: 500,
            timestamp: 12,
            txid: "ch-tx-001".to_owned(),
        }).unwrap();
        tx.send(Transaction {
            amount: 750,
            timestamp: 4,
            txid: "ch-tx-002".to_owned(),
        }).unwrap();
    });

    let mobile_feed2 = spawn(move || {
        tx2.send(Transaction {
            amount: 50,
            timestamp: 7,
            txid: "ch-tx-003".to_owned(),
        }).unwrap();
    });

    file_feed2.join().unwrap();
    mobile_feed2.join().unwrap();


    let mut tl_savings = Account2 {
        acct_type: "Savings".to_owned(),
        account_number: "0001".to_owned(),
        transactions: Vec::new(),
    };

    for transaction in rx {
        tl_savings.transactions.push(transaction);
    }

    println!("mutating from bg threads:\n\t{:?}", tl_savings.transactions);
}


fn version_3() {

    let mut c = Context {
        stop: false,
    };
    let ptr = &mut c as *mut Context;
    let ptr_v = ptr as usize;


    println!("ptr: {:?}", ptr);


    let worker = spawn(move || {
        let ctx = ptr_v as *const Context;

        println!("ctx value: {:?}", ctx);
        

        let wait_millis = time::Duration::from_millis(1 * 1000);
        loop {
            println!("ptr value stop: {}",  unsafe { (*ctx).stop });

            sleep(wait_millis);


            if unsafe { (*ctx).stop } {
                println!("ptr value stop is changed: {}",  unsafe { (*ctx).stop });
                break;
            }
        }
    });



    let wait_millis = time::Duration::from_millis(3 * 1000);
    sleep(wait_millis);


    c.stop = true;
    println!("Set c.stop is true");

   
    worker.join().unwrap();

    println!("End");
}