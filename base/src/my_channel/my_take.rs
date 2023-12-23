#[cfg(test)]
mod test {
    use std::sync::mpsc::channel;
    use threadpool::ThreadPool;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_take_tx() {
        let n_workers = 4;
        let n_jobs = 8;
        let pool = ThreadPool::new(n_workers);
        let (tx, rx) = channel();
        for _ in 0..n_jobs {
            let tx = tx.clone();
            pool.execute(move || {
                tx.send(1).expect("channel will be there waiting for the pool");
            });
        }
        let ret = rx.iter()
            .take(n_jobs)
            .fold(0, |a, b| a + b);
        assert_eq!(ret, 8);
    }

    #[test]
    fn test_tale() {
        let a = vec![12, 3, 4];
        let b = a.into_iter()
            .take(21)
            .collect::<Vec<i32>>();
        assert_eq!(2, b.len());
    }

    #[test]
    fn test_take(){
        let (sender, receiver) = channel();

// Nothing is in the buffer yet
        assert!(receiver.try_iter().next().is_none());
        println!("Nothing in the buffer...");

        thread::spawn(move || {
            for i in 0..110 {
                thread::sleep(Duration::from_secs(1));
                sender.send(i).unwrap();

            }
        });

        println!("Going to sleep...");
        thread::sleep(Duration::from_secs(3)); // block for two seconds
        for x in receiver.try_iter() { // no block
            println!("Got: {x}");
        }

    }
}