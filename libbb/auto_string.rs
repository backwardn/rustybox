use libc;
use libc::chdir;
use libc::chmod;
use libc::chown;
use libc::closelog;
use libc::dup2;
use libc::fstat;
use libc::getenv;
use libc::geteuid;
use libc::getopt;
use libc::getpid;
use libc::isatty;
use libc::kill;
use libc::openlog;
use libc::sigaddset;
use libc::sigemptyset;
use libc::sigprocmask;
use libc::sleep;
use libc::sscanf;
use libc::strcasecmp;
use libc::strcpy;
use libc::symlink;
use libc::syscall;
use libc::syslog;
use libc::time;
use libc::access;
use libc::atoi;
use libc::fclose;
use libc::fprintf;
use libc::lstat;
use libc::printf;
use libc::puts;
use libc::rename;
use libc::rmdir;
use libc::sprintf;
use libc::strchr;
use libc::strcmp;
use libc::strrchr;
use libc::strstr;
use libc::system;



use libc::free;



extern "C" {

}

#[no_mangle]
pub unsafe extern "C" fn auto_string(mut str: *mut libc::c_char) -> *mut libc::c_char {
  static mut saved: [*mut libc::c_char; 4] = [0 as *const libc::c_char as *mut libc::c_char; 4]; /* = 0 */
  static mut cur_saved: u8 = 0;
  free(saved[cur_saved as usize] as *mut libc::c_void);
  saved[cur_saved as usize] = str;
  cur_saved = ((cur_saved as libc::c_int + 1i32) as libc::c_uint
    & ((::std::mem::size_of::<[*mut libc::c_char; 4]>() as libc::c_ulong)
      .wrapping_div(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
      as libc::c_uint)
      .wrapping_sub(1i32 as libc::c_uint)) as u8;
  return str;
}
