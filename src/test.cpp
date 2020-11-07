#include <stdio.h>

#include <thread>

typedef void LOG_FOO_LOCAL(int);

// 存储log_foo_local的全局变量
void (*RUST_LOG_FOO_LOCAL)(int);

// 新线程运行的函数
void try_log_thread_local() {
  RUST_LOG_FOO_LOCAL(666666);
}

extern "C" void cpp_foo_test(LOG_FOO_LOCAL rust_log_fn) {
  rust_log_fn(2333333);

  // 将收到的函数赋值给全局变量
  RUST_LOG_FOO_LOCAL = rust_log_fn;

  std::thread threadObj(try_log_thread_local);

  threadObj.join();
}































































