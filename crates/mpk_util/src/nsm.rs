//! MPK_UTIL NSM utils
use std::io::Error;
use std::net::{SocketAddr, ToSocketAddrs};

#[cfg(target_os = "linux")]
pub fn start_nsmd(args: &[&str]) -> Result<std::process::Child, Error> {
  std::process::Command::new("nsmd")
    .args(args)
    .spawn()
    .map_err(|e| e.into())
}

/// Lazily get the PID for a running nsmd server.
///
/// Note that this command will return only the first line of the
/// output of `pgrep`, so if there are multiple servers running you
/// will need to obtain the PID elsewhere.
pub fn nsmd_pid() -> String {
  let pid = String::from_utf8(
    std::process::Command::new("pgrep")
      .args(["-f", "nsmd"])
      .output()
      .unwrap()
      .stdout,
  )
  .unwrap();
  if !pid.is_empty() {
    pid.lines().next().unwrap().to_string()
  } else {
    panic!("could not find pid for nsmd. make sure to start it first.")
  }
}

/// Parse a NSM_URL of form `osc.udp://HOST:PORT/` and return a
/// SocketAddr.
pub fn parse_nsm_url(url: &str) -> Result<SocketAddr, Error> {
  match url
    .strip_prefix("osc.udp://")
    .unwrap()
    .strip_suffix("/\n")
    .unwrap()
    .to_socket_addrs()
  {
    Ok(mut s) => Ok(s.next().unwrap()),
    Err(e) => Err(e),
  }
}

/// Return the NSM_URL given the PID of a running nsmd server and return
/// a SocketAddr. The server needs to be started by the same user
/// executing this function. The NSM_URL is parsed from the daemon
/// state file at `/run/user/UID/nsm/d/PID`.
pub fn get_nsm_url(pid: &str) -> Result<SocketAddr, Error> {
  let id = String::from_utf8(
    std::process::Command::new("id")
      .arg("-u")
      .output()
      .unwrap()
      .stdout,
  )
  .unwrap();
  let url =
    std::fs::read_to_string(format!("/run/user/{}/nsm/d/{}", id.trim(), pid)).unwrap();
  parse_nsm_url(&url)
}
