mod utils;

extern "C" {
  pub fn open_window();
}

fn main() {

	unsafe {
		open_window();
	}
}
