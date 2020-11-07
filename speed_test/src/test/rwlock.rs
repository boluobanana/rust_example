use std::sync::{Mutex, RwLock};
use std::time::Instant;

lazy_static!{
  static ref RWLOCK_INSTANCE_WRITE: RwLock<u32> = RwLock::new(0);
  static ref RWLOCK_INSTANCE_READ: RwLock<u32> = RwLock::new(0);
}

#[test]
fn benchmark_rwlock_write() {
  let now = Instant::now();
  for i in 0..1_000_000 {
    let mut data= RWLOCK_INSTANCE_WRITE.write().unwrap();
    *data += 1;
  }
  let elapsed = now.elapsed();
  println!("rwlock write 1_000_000 times cost: {:?} one time: {:?}", elapsed, elapsed / 1_000_000);
}

#[test]
fn benchmark_rwlock_read() {
  let now = Instant::now();
  for i in 0..1_000_000 {
    let data= RWLOCK_INSTANCE_READ.read().unwrap();
  }
  let elapsed = now.elapsed();
  println!("rwlock read 1_000_000 times cost: {:?} one time: {:?}", elapsed, elapsed / 1_000_000);
}


