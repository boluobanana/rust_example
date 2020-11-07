use std::sync::{Mutex, RwLock};
use std::time::Instant;
use std::cell::{RefCell, Cell};
use std::sync::Arc;
use parking_lot::{RwLock as LotRwLock};

lazy_static!{
  static ref RWLOCK_INSTANCE_LOT_READ: LotRwLock<u32> = LotRwLock::new(0);
  static ref RWLOCK_INSTANCE_LOT_WRITE: LotRwLock<u32> = LotRwLock::new(0);
}

#[test]
fn benchmark_lot_rwlock_write() {
  let now = Instant::now();
  for i in 0..1_000_000 {
    let mut data= RWLOCK_INSTANCE_LOT_WRITE.write();
    *data += 1;
  }
  let elapsed = now.elapsed();
  println!("parking lot rwlock write 1_000_000 times cost: {:?} one time: {:?}", elapsed, elapsed / 1_000_000);
}

#[test]
fn benchmark_lot_rwlock_read() {
  let now = Instant::now();
  for i in 0..1_000_000 {
    let data= RWLOCK_INSTANCE_LOT_READ.read();
  }
  let elapsed = now.elapsed();
  println!("parking lot  rwlock read 1_000_000 times cost: {:?} one time: {:?}", elapsed, elapsed / 1_000_000);
}
