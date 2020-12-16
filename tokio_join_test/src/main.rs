//! 测试tokio::main和tokio::main(basic_scheduler)，任务的并发处理情况。
//! 开启线程池后，任务会并发执行，basic的话会一次执行
//!
//! tokio::join会并行的执行两个任务，但是依旧有执行的先后顺序
//!
//!
use std::sync::{Arc, atomic::{AtomicI32, Ordering}};

use tokio::{join, sync::mpsc::{unbounded_channel, UnboundedReceiver}};

fn main() {

    let (tx, rx) = unbounded_channel::<i32>();
    println!("Hello, world!");

    let t = std::thread::spawn(move ||{
        run_tokio(rx);
    });

    let tx2 = tx.clone();
    std::thread::spawn(move|| {
        for i in 0..30 {
            tx2.send(i).unwrap();
        }
    });

    // let tx2 = tx.clone();
    // std::thread::spawn(move|| {
    //     for i in 124..234 {
    //         tx2.send(i).unwrap();
    //     }
    // });


    t.join().unwrap();

}

#[tokio::main(basic_scheduler)] // 顺序执行，不会输出 wrong order
// #[tokio::main]               // 并行执行，会输出wrong order
async fn run_tokio(rx: UnboundedReceiver<i32>) {
    let mut rx = rx;
    let current_run_id = Arc::new(AtomicI32::new(0));
    loop {
        let current_run_id_clone = current_run_id.clone();
        match rx.recv().await {
            Some(x) => {
                tokio::spawn(async move {
                    let f1 = async {
                        println!("f1 {}",x);
                        if current_run_id_clone.load(Ordering::Relaxed) == x {
                            println!("rev: {}", x);
                            current_run_id_clone.store(x + 1, Ordering::Relaxed);
                            async {
                                x
                            }.await;
                        } else {
                            println!("!!!!!!!!!!!!!! wrong order {:?}, x: {}", current_run_id_clone, x);
                        }
                    };

                    let f2 = async {
                        println!("f22222 {}", x);
                        x
                    };

                    // 依次去执行，一起返回后返回
                    join!(f2, f1);
                });
            },
            None => {
                break;
            }
        }
    }
}
