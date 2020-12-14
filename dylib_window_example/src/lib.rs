mod window;

#[no_mangle]
pub extern "C" fn open_window() {
    window::create_and_open();
}