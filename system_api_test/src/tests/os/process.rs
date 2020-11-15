use crate::os::process::terminate_process;

#[test]
fn test_terminate_process() {
  /// 测试时需手动修改进程id来测试
  let pid = 7266;
  terminate_process(pid).expect("test_terminate_process");
}