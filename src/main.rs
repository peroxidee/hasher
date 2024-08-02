use whirlpool::{Whirlpool, Digest};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();


    for arg in args.iter(){
    
    
            let mut hasher = Whirlpool::new();
    
            
            hasher.update(arg.as_bytes());
    
            let result = hasher.finalize();
    
            println!("your hash is: {:x}", result)
    
    }

}
