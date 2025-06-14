use std::{time::Duration, thread::sleep};
const CLEAR: &str = "\x1B[2J\x1B[1;1H";
fn calculation(){
    sleep(Duration::from_secs(1));
    println!("abc");
    }

fn progress<T>(v:Vec<T>,f:fn()){
    let mut pro = 1;
   for n in  v.iter(){
    println!("{}{}",CLEAR,"*".repeat(pro));
    pro +=1;
    f();

}}
fn main() {
   let vector = vec![1,2,3,4,5,6,7,8,9,10];
   
    progress(vector, calculation);
   
}
