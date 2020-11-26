use std::sync::Mutex;

#[macro_use]
extern crate lazy_static;



fn init_lazy_static() {
    lazy_static! {
        static ref TEST_STATIC: Mutex<String> = {
          Mutex::new("tewts".to_string())
        };
    }
}

fn try_read_static() {
    // TEST_STATIC.lock().unwrap()
}

fn main() {
    init_lazy_static();

    try_read_static();
}
