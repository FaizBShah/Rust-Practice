// Concurrency
use std::thread;
use std::time::Duration;
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};

pub struct Bank {
    balance: f32
}

// fn withdraw(bank: &mut Bank, amt: f32) {
//     bank.balance -= amt;
// }

// fn customer(bank: &mut Bank) {
//     withdraw(bank, 5.0);
// }

fn withdraw(bank: &Arc<Mutex<Bank>>, amt: f32) {
    let mut bank_ref = bank.lock().unwrap();

    if bank_ref.balance < 5.00 {
        println!("Current balance: {}. Withdraw a smaller amount", bank_ref.balance);
    } else {
        bank_ref.balance -= amt;
        println!("Customer withdrew: {}. Current balance: {}", amt, bank_ref.balance);
    }
}

fn customer(bank: Arc<Mutex<Bank>>) {
    withdraw(&bank, 5.0);
}

fn main() {
    // let thread1 = thread::spawn(|| {
    //     for i in 1..25 {
    //         println!("Spawned thread: {}", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });

    // for i in 1..20 {
    //     println!("Main thread: {}", i);
    //     thread::sleep(Duration::from_millis(1));
    // }

    // // Waits for the thread1 to finish even if the main thread finishes
    // thread1.join().unwrap();

    // let mut bank = Bank { 
    //     balance: 100.0 
    // };

    // withdraw(&mut bank, 5.0);

    // This is giving error as bank cannot be borrowed from main function
    // thread::spawn(|| {
    //     customer(&mut bank);
    // }).join().unwrap();

    // println!("Balance: {}", bank.balance);

    let bank = Arc::new(Mutex::new(Bank { balance: 20.0 }));

    let handles = (0..10).map(|_| {
        let bank_ref = bank.clone();
        thread::spawn(|| {
            customer(bank_ref);
        })
    });

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Total balance: {}", bank.lock().unwrap().balance);
}
