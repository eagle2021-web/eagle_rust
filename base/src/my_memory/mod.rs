#[cfg(test)]
mod test {
    #[test]
    fn test_a(){
        let x = vec![1, 2, 3, 4, 4, 5, 5];
        unsafe {
            let t: (usize, usize, usize) = std::mem::transmute_copy(&x);
            // 堆大小 內存大小 元素个数
            println!("{}, {}, {}", t.0, t.1, t.2);
        }
    }
}