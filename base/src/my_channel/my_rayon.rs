fn partition<T: PartialOrd + Send>(v: &mut [T]) -> usize {
    let pivot = v.len() - 1;
    let mut i = 0;
    for j in 0..pivot {
        if v[j] <= v[pivot] {
            v.swap(i, j);
            i += 1;
        }
    }
    v.swap(i, pivot);
    i
}

fn quick_sort<T: PartialOrd + Send>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }
    let mid = partition(v);
    let (lo, hi) = v.split_at_mut(mid);
    // rayon::join(||
                    quick_sort(lo);
                // , ||
                    quick_sort(hi);
    // );
}


#[cfg(test)]
mod test {
    use super::quick_sort;

    #[test]
    fn test_quick_sort() {
        let mut v = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        quick_sort(&mut v);
        let b = v.iter()
            .zip(v.iter().skip(1))
            .all(|(&a, &b)|{
                a < b
            });
        assert!(b);
        println!("{:?}", v);
    }
}