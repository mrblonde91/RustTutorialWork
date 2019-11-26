//16.02 Channels
use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    // mpsc - multiple producer, single consumer
    let (tx, rx) = mpsc::channel();

    // clone the transmitter
    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("My"),
            String::from("name"),
            String::from("Shady")
            ];
        //needs to take ownership of transmitter
        for val in vals {
        tx.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
        }
        //Cannot print val as lost ownership
        //this prevents modifications after sent from thread
        //println!("Val is {}" val);
    });

thread::spawn(move || {
        let vals = vec![
            String::from("further"),
            String::from("messages"),
            String::from("in another thread"),
            String::from("for the receiver to adopt")
            ];
        for val in vals {
        tx1.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx{
    println!("Got: {}", received);
    }
}
