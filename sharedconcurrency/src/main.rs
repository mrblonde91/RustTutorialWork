use std::sync::{Mutex, Arc};
use std::thread;
//16.03 Shared concurrency
// Mutexes require a lock to be acquired and only one can have at any given time
fn main() {
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
        //lock drops after exits scope
    }
    println!("m = {:?}", m);

    //Arc is concurrent equivalent of Rc
    let counter = Arc::new(Mutex::new(0));
    let mut handles  =vec![];

    for _ in 0..10{
        //counts number of references to counter so becomes safe
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }



    for handle in handles{
        handle.join().unwrap()
    }

    println!("Result: {}", *counter.lock().unwrap());
}
