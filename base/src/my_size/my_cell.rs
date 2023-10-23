#[cfg(test)]
mod test{
    use crate::assert_size;
    use std::cell::Cell;
    #[test]
    fn test_cell(){
        assert_size!(Cell<i32>, 4);
        assert_size!(Cell<i8>, 1);
    }
}