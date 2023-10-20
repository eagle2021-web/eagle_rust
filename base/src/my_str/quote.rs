pub fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
#[cfg(test)]
mod test {
    use super::first_word;

    #[test]
    fn test_a() {
        let mut s = String::from("hello");
        let _word = first_word(&s);
        s.clear();
        // 此時word不能使用，因爲s.clear用了mut
    }
}
