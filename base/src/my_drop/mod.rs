struct D{
     name: String
}
 impl Drop for D {
     fn drop(&mut self) {
         println!("drop d {}", &self.name);
     }
 }
 #[cfg(test)]
 mod test{
     use crate::my_drop::D;

     #[test]
     fn test1(){
         let d = D{
             name: "SS".to_string()
         };
         drop(d);
         println!("00");
     }
 }