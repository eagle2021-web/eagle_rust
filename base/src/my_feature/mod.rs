mod sleep_future;

#[cfg(test)]
mod test{
    use std::sync::Arc;

    #[test]
    fn test1(){
        struct State {
            name: String,
        }
        let s = State {
            name: "asdfa".to_string()
        };
        let a = Arc::new(&s);
        let b = Arc::clone(&a);
        let c = Arc::new(&s);
    }
}