use std::{time::Duration, thread::sleep};
fn calculation(_n:&i32){
    sleep(Duration::from_secs(1));
    println!("more and more");

}
fn main() {
   let vector = vec![1,2,3,4,5,6,7];
   for n in  vector.iter(){
    calculation(n);
   }
}
