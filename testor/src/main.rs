use std::{time::Duration, thread::sleep};
struct Progress<Iter>{
    iter:Iter,
    i:usize,
}
impl<Iter> Progress<Iter>{
    pub fn new(iter:Iter)->Self{
        Progress{iter,i:0}
    }
}
impl<Iter> Iterator for Progress <Iter>
    where Iter:Iterator {
        type Item = Iter::Item;
            fn next(&mut self)-> Option<Self::Item> {
                println!("{}{}",CLEAR,"#".repeat(self.i));
                self.i +=1;
                self.iter.next()
    }
}
trait ProgressIteratorExtension:Sized{
    fn progress(self)->Progress<Self>;
}
impl<Iter>ProgressIteratorExtension for Iter{
    fn progress(self)->Progress<Self>{
        Progress::new(self)
    }
}

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
fn progress_h<T,  Iter>(it:Iter, f:fn())
where Iter: Iterator<Item = T>{
    let mut pro = 1;
    for n in  it{
    println!("{}{}",CLEAR,"*".repeat(pro));
    pro +=1;
    f();
    }
}


fn main() {
    use std::collections::HashSet;
    let mut hashi =HashSet::new();
    hashi.insert(0);
    hashi.insert(1);
    hashi.insert(2);
   let vector = vec![1,2,3,4,5,6,7,8,9,10];
   
    progress(vector, calculation);
    progress_h(hashi.iter(), calculation);

     let vr = vec![1,2,3,4,5,6,7,8,9,10];
     for _ in Progress::new(vr.iter()) {
        calculation();
     }
     let vt = vec![1,2,3,4,5,6,7,8,9,10];
     for _ in vt.iter().progress(){
        calculation();
     }

   
}
