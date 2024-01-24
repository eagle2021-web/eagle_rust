extern crate crossbeam;
#[macro_use]
use crossbeam::channel;
use channel::{Receiver, Sender};
use crossbeam::select;
fn seek<'a>(name: &'a str, tx: &Sender<&'a str>, rx: &Receiver<&'a str>) {
    select! {
            recv(rx) -> res => println!("{} received a message from", name),
            send(tx, name) -> res => println!("{} send.", name),
    }
}

#[cfg(test)]
mod test {
    use std::thread;
    use std::thread::sleep;
    use std::time::Duration;
    use super::select;
    use super::seek;
    #[test]
    fn test_take_tx() {
        let people = vec!["Anna", "Bob", "Cody", "Dave", "Eva", "eagle", "wy", "ggb", "zyx"];
        let (tx, rx) = crossbeam::channel::bounded(4); // Make room for one unmatched send.
        let (tx, rx) = (&tx, &rx);
        crossbeam::scope(|s| {
            for name in people {
                s.spawn(|_| seek(name, tx, rx));
            }
        });

        if let Ok(name) = rx.try_recv() {
            println!("{}'s message left in channel.", name);
        }
    }

    #[test]
    fn test_scope_sleep(){
        let people = vec!["Anna", "Bob", "Cody", "Dave", "Eva", "eagle", "wy", "ggb", "zyx"
        ,"Anna", "Bob", "Cody", "Dave", "Eva", "eagle", "wy", "ggb", "zyx"];
        let (tx, rx ) = crossbeam::channel::bounded(2);
        let (tx2, rx2) = (&tx, &rx);
        crossbeam::scope(|s| {
            for name in people {
                s.spawn(|_| {
                    seek(name, &tx, &rx);
                    // sleep(Duration::from_secs(2));
                    println!("Hello, I'm thread: {:?}", std::thread::current().id());
                });
            }
        });

        if let Ok(name) = rx.try_recv() {
            println!("{}'s message left in channel.", name);
        }
    }
    #[test]
    fn test_a(){

    }
    #[test]
    fn channel() {
        let (s, r) = crossbeam::channel::bounded(1);

        select! {
            send(s, 0) -> res =>{println!("1"); res.unwrap()} ,
            recv(r) -> res => {
                println!("2");
                assert!(res.is_ok())
            },
        }
    }
    const COUNT: u32 = 10;
    #[test]
    fn channel22() {
        let (s, r) = crossbeam::channel::bounded(1);
        let (s2, r2) = crossbeam::channel::bounded(1);
        let thread1 = thread::spawn(move || {
            for i in 0..COUNT {
                s.send(1);
                print!("{}", r2.recv().unwrap());
            }
        });
        let thread2 = thread::spawn(move || {
            for i in 0..COUNT {
                print!("{}", r.recv().unwrap());
                s2.send('a');
            }
        });
        thread1.join().ok();
        thread2.join().ok();
        // println!("111111");
    }
}