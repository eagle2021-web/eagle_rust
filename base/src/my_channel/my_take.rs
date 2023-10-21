#[cfg(test)]
mod test {
    use std::sync::mpsc::channel;
    use threadpool::ThreadPool;

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
            .take(2)
            .collect::<Vec<i32>>();
        assert_eq!(2, b.len());
    }
}