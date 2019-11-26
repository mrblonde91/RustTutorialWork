use std::thread;
use std::time::Duration;
//16.01 threading
fn main() {
    let handle = thread::spawn(|| {
        //This will stop when full scopes out
        for i in 1..10{
            println!("HI Number {} from the spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

            for i in 1..5{
            println!("HI Number {} from the  main thread", i);
            thread::sleep(Duration::from_millis(1));
        }

    //To allow spawned thread to complete, run a join on handle, this blocks the thread allowing for completion
    handle.join().unwrap();

    //using move to force ownership
    let v = vec![1,2,3];
    let secondhandler = thread::spawn(move|| {
        println!("Heres a vector {:?}", v);
    });
    //won't be allowed to drop value
    //drop(v);
    secondhandler.join().unwrap();
}
