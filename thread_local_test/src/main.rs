use std::cell::RefCell;
thread_local! {
    pub static FOO_LOCAL: RefCell<u32> = RefCell::new(444);
}

extern "C" fn log_foo_local(from: i32) {
    FOO_LOCAL.with(|foo| {
        println!("thread: {:?} foo local is {:?}", from, foo);
    });
}
type LogFoo = extern "C" fn(i32);
extern {
    fn cpp_foo_test(foo: LogFoo);
}

fn main() {
    FOO_LOCAL.with(|foo| {
        // 在主线程修改FOO_LOCAL值未1
        foo.replace(1);
        log_foo_local(1)
    });

    std::thread::spawn(|| {
        FOO_LOCAL.with(|foo| {
            // 在新线程修改FOO_LOCAL值未2
            foo.replace(2);
            log_foo_local(2);
        });
        // 将log_foo_local函数传入cpp内，有cpp直接调用
        unsafe  {
            cpp_foo_test(log_foo_local);
        }
    });
}


