fn main() {}

#[cfg(test)]
mod test {
    use std::collections::BinaryHeap;
    use std::cmp::{Ordering, Reverse};

    #[test]
    fn test_greater_root() {
        let mut max_heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();

        max_heap.push(Reverse(5));
        max_heap.push(Reverse(3));
        max_heap.push(Reverse(7));

        while let Some(Reverse(val)) = max_heap.pop() {
            println!("{}", val);
        }
        let a = Reverse(1);
        let b: Reverse<i32> = Reverse(12);
        match a.cmp(&b) {
            Ordering::Less => {
                println!("smaller");
            }
            Ordering::Equal => {
                println!("eq");
            }
            Ordering::Greater => {
                println!("greater");
            }
        }
    }

    #[test]
    fn test_smaller_root() {
        let mut min_heap: BinaryHeap<i32> = BinaryHeap::new();

        min_heap.push(5);
        min_heap.push(3);
        min_heap.push(7);

        while let Some(val) = min_heap.pop() {
            println!("{}", val);
        }

        let mut v = vec![1, 2, 3, 4, 5, 6, 7];
        v.sort_by_key(|&num| (num > 5, Reverse(num)));
        println!("{:?}", v);
        // assert_eq!(v, vec![3, 2, 1, 6, 5, 4]);
    }
}