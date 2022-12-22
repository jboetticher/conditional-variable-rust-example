use std::{
    sync::{Arc, Condvar, Mutex},
    thread::{self, sleep}, time::Duration
};

const POT_SIZE: usize = 3;
const CRABS_TO_COOK: i32 = 3 * 30;

fn main() {
    let prepared_crabs: Mutex<Vec<String>> = Mutex::new(Vec::<String>::new());
    // let cooked_crabs_num: Mutex<i32> = Mutex::new(0);
    let enough_for_pot = Condvar::new();

    // Produce 2 atomic reference counters. One for the prepare thread and one for
    // the cook thread. They're reference counters, so they point to the same data!
    let prepare = Arc::new((
        prepared_crabs,
        //cooked_crabs_num,
        enough_for_pot,
    ));
    let cook = Arc::clone(&prepare);
    let main = Arc::clone(&prepare);

    // Prepare the 🦀 (producers)
    let producer_handle = thread::spawn(move || {
        for _i in 0..CRABS_TO_COOK {
            let (prepared_lock /*,finished_lock*/, enough_for_pot) = &*prepare;

            let mut prepared_crabs = prepared_lock.lock().unwrap();
            prepared_crabs.push(String::from("🦀"));

            // Explicity unlock
            println!("Prepared a 🦀! Number prepared: {}", prepared_crabs.len());

            // If there are enough prepared, then it's time to notify the cookers
            if prepared_crabs.len() >= POT_SIZE {
                enough_for_pot.notify_one();
            }
            drop(prepared_lock);

            sleep(Duration::from_micros(10000));
        }
    });

    // Cook the 🦀 (consumers)
    let consumer_handle = thread::spawn(move || {
        let mut cooked_crabs = 0;
        loop {
            let (prepared_lock /*,finished_lock*/, enough_for_pot) = &*cook;

            let mut prepared_crabs = prepared_lock.lock().unwrap();
            if prepared_crabs.len() < POT_SIZE {
                prepared_crabs = enough_for_pot.wait(prepared_crabs).unwrap();
            }

            if prepared_crabs.len() < POT_SIZE {
                println!("Spurrious wakeup!");
            } else {
                // Put them into the pot!
                prepared_crabs.pop();
                prepared_crabs.pop();
                prepared_crabs.pop();
                cooked_crabs += 3;
                println!("Added 3 🦀 to the pot: {:?}", prepared_crabs);
            }
            sleep(Duration::from_micros(10000));

            if cooked_crabs == CRABS_TO_COOK { 
                break;
            }
        }
    });

    // Join the threads to ensure that everything is done
    match consumer_handle.join() {
        Ok(_) => (),
        Err(err) => println!("{:?}", err),
    }

    match producer_handle.join() {
        Ok(_) => (),
        Err(err) => println!("{:?}", err),
    }

    println!("After Join: {:?}", main.0);
}