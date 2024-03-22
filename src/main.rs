//!*The program calculates any piece of fibonacci series then sorts it's related piece(e.g  it sorts 0 1 2 3 5 8 for fibonacci(5)) 
//! unless index of requested elementh is less than 48
//! Otherwise program will return the first fibonacci number which is equal to 1
//

use std::io;
trait Countable{
    fn new(&self)->Self;
    fn find_elementh(&self,n:u32)->u32;
    fn sort(&self,n:u32)->Vec<u32>;
}
///*Fibonnaci is debugable and it gets two parameters consist of positive 32 bits integers(u32):
/// index : the index of fibonacci number
/// result : the fibonacci number that is located in the corresponding index
/// Consider calling this struct with following syntax new(&Fibonacci{index:number,result:number}) where number is any variable in u32 type
/// In constructor result can be equal to any positive integer 
/// because it will be automatically replaced with  the right number by calling find_elementh function by the program itself
/// if user constructs this struct via new method*/

#[derive(Debug)]
struct Fibonacci{
    index:u32,
    result:u32
}

///implements Countable trait to Fibonacci struct however Countable trait is optional because impl Fibonacci{...} will work as well
impl Countable for Fibonacci{
    fn find_elementh(&self,n:u32)->u32{
        let mut sum:u32 =0; 
        let mut first=0;
        let mut second=1;
        if n<=1{
            return n;
        }
        else{
          for _ in 1..n{
            sum = first + second;
            first = second;
            second = sum;
            }
            sum  
        }
    }
    #[allow(unused_assignments)]
    fn sort(&self,n:u32)->Vec<u32>{
        let mut elems= Vec::new();
        let mut sum:u32 =0; 
        let mut first=0;
        let mut second=1;
        if n<=1{
            elems.push(0);
            if n>0 {elems.push(1)} else{}
            return elems;
        }
        else{
          elems.push(0);
          elems.push(1);
          for _ in 1..n{
            sum = first + second;
            first = second;
            second = sum;
            elems.push(sum)
            }
            elems
        }
    }
    fn new(&self)->Self{
        Self{
            index : self.index,
            result : self.find_elementh(self.index)
        }
    }
    
}

///Program will get a positive integer from user then sort the corresponding fibonacci values if user input is valid
fn main() {
    let mut answer:String=String::new();
    let _ = io::stdin().read_line(&mut answer);
     match answer.trim().parse::<u32>(){
        Ok(mut n)=>{
            n = if n > 47 {1} else {n};
            let number : Fibonacci = Fibonacci::new(&Fibonacci{index:n,result:0});
            println!("\n{:#?}",number);
            println!("\nfibonacci({}) = {}",n,number.result);
            let elemenths = number.sort(n);
            let _ = elemenths.iter().collect::<Vec<&u32>>();
            println!("\nFirst {} elemenths of fibonacci series: {:?}",n,elemenths);
        }
        Err(n)=>{
            print!("\n{}\n",n);
        }
    }
}
