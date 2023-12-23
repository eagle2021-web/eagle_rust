use std::sync::mpsc;
use std::thread;
#[cfg(test)]
mod test {
    use std::sync::mpsc;
    use std::thread;
    #[test]
    fn test_a(){
        // 使用标准库的 channel 实现。
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            tx.send(10).unwrap();
        });
        let a = rx.recv().unwrap();
        println!("Received: {}", a);

        // 使用 Crossbeam 的 channel 实现。
        let (s, r) = crossbeam::channel::bounded(1);

        thread::spawn(move || {
            s.send(20).unwrap();
            s.send(21).unwrap();
        });

        println!("Received: {}", r.recv().unwrap());
        println!("Received: {}", r.recv().unwrap());
    }
}