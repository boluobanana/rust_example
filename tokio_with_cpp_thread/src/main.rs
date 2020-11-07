extern "C" fn log_foo_local(from: i32) {

}

type SpawnTokioTask = extern "C" fn(i32);

extern {
    fn cpp_foo_test(foo: SpawnTokioTask);
}

fn main() {

    let mut rt = tokio::runtime::Builder::new().threaded_scheduler().enable_all().build().unwrap();

    rt.block_on(async {
        let loop_fut = log_loop();

        unsafe  {
            cpp_foo_test(spawn_tokio_task);
        }
        spawn_tokio_task(1);

        std::thread::spawn(|| {
            spawn_tokio_task(444);
        });
        tokio::join!(loop_fut);
    });

}

async fn log_loop() {
    loop {
        println!("loop");
        tokio::time::delay_for(std::time::Duration::from_millis(5000)).await;
    }
}

extern "C" fn spawn_tokio_task(id: i32) {
    tokio::spawn(async move {
        tokio::time::delay_for(std::time::Duration::from_millis(1000)).await;

        println!("spawn_tokio_task id: {}", id);
    });
}
