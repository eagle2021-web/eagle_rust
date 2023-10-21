
pub fn add(left: usize, right: usize) -> usize {
    left + right
}
pub fn replace_str(p: &str) -> String {
    p.replace("\\", "/")
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_replace() {
        let s:&str = "d:/eagle_repos/gitee.com/zengyong2020/web-editor-markdown.git\\v1.0.6";
        let p_buf = PathBuf::from(s);
        let p_path = p_buf.as_path();
        let _os_str = p_path.as_os_str();
    }
}
