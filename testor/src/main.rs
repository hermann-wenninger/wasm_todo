use std::{time::Duration, thread::sleep};
struct Progress<Iter>{
    iter:Iter,
    i:usize,
    bound: Option<usize>,
    delims:(char,char),
}

impl<Iter>Progress<Iter>
    where Iter: ExactSizeIterator{
        pub fn with_bound(mut self) -> Self {
            self.bound = Some(self.iter.len());
            self
        }
}

impl<Iter>Progress<Iter>{
    pub fn with_delims(mut self, delims:(char,char))->Self{
        self.delims = delims;
        self
    }
}

impl<Iter> Progress<Iter>{
    pub fn new(iter:Iter)->Self{
        Progress{iter,i:0,bound:None,delims:('[', ']')}
    }
}




impl<Iter> Iterator for Progress <Iter>
    where Iter:Iterator {
        type Item = Iter::Item;
            fn next(&mut self)-> Option<Self::Item> {
                print!("{}",CLEAR);
                match self.bound{
                    Some(bound) =>
                            println!("{}{}{}{}",
                            self.delims.0,
                            "*".repeat(self.i),
                            " ".repeat(bound - self.i),
                            self.delims.1),
                    None => {
                            println!("{}","*".repeat(self.i));
                    }
                }
                //println!("{}{}",CLEAR,"#".repeat(self.i));
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
  
    }

fn progress<T>(v:Vec<T>,f:fn()){
    let mut pro = 1;
    for _n in  v.iter(){
    println!("{}{}",CLEAR,"*".repeat(pro));
    pro +=1;
    f();
}}
fn progress_h<T,  Iter>(it:Iter, f:fn())
where Iter: Iterator<Item = T>{
    let mut pro = 1;
    for _n in  it{
    println!("{}{}",CLEAR,"*".repeat(pro));
    pro +=1;
    f();
    }
}


fn main() {
    let brakts = ('<','>');
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
     let kato = vec![1,2,3,4,5,6,7,8,9,10];
     for _n in kato.iter().progress().with_bound().with_delims(brakts){
        calculation();
     }
   
}
