//! 这里是像测试直接访问一个bool值的时间和访问一个atomic bool的时间的差距直接访问是42ns, atomic是58ns
//!
//!

use std::sync::{Mutex, RwLock, atomic::{AtomicBool, Ordering}};
use std::time::Instant;



static ATOMIC_BOOL_INSTANCE: AtomicBool = AtomicBool::new(false);

static mut GLOBAL_DATA: bool = true;

#[test]
fn benchmark_atomic_bool() {
  let now = Instant::now();
  for i in 0..1_000_000 {
    let data = ATOMIC_BOOL_INSTANCE.load(Ordering::SeqCst);
  }

  let elapsed = now.elapsed();
  let data = ATOMIC_BOOL_INSTANCE.load(Ordering::SeqCst);
  println!("atomic bool 1_000_000 times cost: {:?} one time: {:?}, {:?}", elapsed, elapsed / 1_000_000, data);

  let now = Instant::now();

  for i in 0..1_000_000 {
    unsafe  {
      GLOBAL_DATA;
    }
  }

  let elapsed = now.elapsed();
  println!("just visit 1_000_000 times cost: {:?} one time: {:?}, ", elapsed, elapsed / 1_000_000);
}

