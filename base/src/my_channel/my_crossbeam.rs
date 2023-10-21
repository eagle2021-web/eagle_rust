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

    use super::select;
    use super::seek;
    #[test]
    fn test_take_tx() {
        let people = vec!["Anna", "Bob", "Cody", "Dave", "Eva"];
        let (tx, rx) = crossbeam::channel::bounded(1); // Make room for one unmatched send.
        let (tx, rx) = (&tx, &rx);
        crossbeam::scope(|s| {
            for name in people {
                s.spawn(move |_| seek(name, tx, rx));
            }
        });
        if let Ok(name) = rx.try_recv() {
            println!("No one received {}’s message.", name);
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
}