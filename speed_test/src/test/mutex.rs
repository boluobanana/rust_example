use std::sync::{Mutex, RwLock};
use std::time::Instant;

lazy_static!{
  static ref MUTEX_INSTANCE: Mutex<u32> = Mutex::new(0);
}

static mut GLOBAL_DATA: u32 = 0;

#[test]
fn benchmark_mutex() {
  let now = Instant::now();
  for i in 0..1_000_000 {
    let mut data = MUTEX_INSTANCE.lock().unwrap();
    *data += 1;
  }
  let elapsed = now.elapsed();
  let data = MUTEX_INSTANCE.lock().unwrap();
  println!("mutex 1_000_000 times cost: {:?} one time: {:?}, {:?}", elapsed, elapsed / 1_000_000, data);

  let now = Instant::now();

  for i in 0..1_000_000 {
    unsafe  {
      GLOBAL_DATA += 1;
    }
  }
  let elapsed = now.elapsed();
  println!("just visit 1_000_000 times cost: {:?} one time: {:?}, ", elapsed, elapsed / 1_000_000);
}

