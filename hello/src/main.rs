mod collection;

use std::{thread::spawn, panic};

use collection::{list::List, Queue};

type Error = Box<dyn std::error::Error>;
fn panic_func() -> Result<i32,Error> {
    let v = vec![1,2,3];
    Ok(v[100])
} 

fn main() {
    let panicing_task = spawn(|| {
        let mut val = 666_u32;
        for i in 0..5 {
            val = val / i;
        }
        val
    });
    // let res = panic::catch_unwind(|| {
    //     let mut val = 666_u32;
    //     for i in 0..5 {
    //         val = val / i;
    //     }
    //     val
    // });
    // match panic_func() {
    //     Ok(v) => println!("Not Paniced"),
    //     Err(e) => println!("PAA => {:?}",e)
    // };
    let res = panic_func();
    // let res = panicing_task.join();
    match res {
        Ok(v) => println!("value {}", v),
        Err(e) => println!("PANIC => : {:?}", e)
    };

}

