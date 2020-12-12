mod window;

pub extern "C" fn open_window() {
    window::create_and_open();
}