#include <stdio.h>

#include <thread>

typedef void LOG_FOO_LOCAL(int);

// 存储log_foo_local的全局变量
void (*RUST_SPAWN_TOKIO_TASK)(int);

// 新线程运行的函数
void try_log_thread_local() {
  RUST_SPAWN_TOKIO_TASK(666666);
}

extern "C" void cpp_foo_test(LOG_FOO_LOCAL spawn_tokio_task) {
  spawn_tokio_task(2333333);

  // 将收到的函数赋值给全局变量
  // RUST_SPAWN_TOKIO_TASK = spawn_tokio_task;

  // std::thread threadObj(try_log_thread_local);

  // threadObj.join();
}































































