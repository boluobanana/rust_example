use std::{io, process::{Command, Output}};

#[cfg(target_os="win")]
use winapi::um::winnt::HANDLE;


/// 通过kill -s KILL <PID>的方式来杀死进程
#[cfg(target_os="macos")]
pub fn terminate_process(pid: i64) -> io::Result<Output>{
  Command::new("kill")
  .args(&["-s", "KILL", &pid.to_string()])
  .output()
}

#[cfg(target_os="win")]
pub fn terminate_process(win_handle: HANDLE, exit_code: u64)  -> bool {
  winapi::um::processthreadsapi::TerminateProcess(win_handle, exit_code)
}
