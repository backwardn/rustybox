use crate::archival::libarchive::bb_archive::archive_handle_t;
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




use libc::off_t;

#[no_mangle]
pub unsafe extern "C" fn data_align(
  mut archive_handle: *mut archive_handle_t,
  mut boundary: libc::c_uint,
) {
  let mut skip_amount: libc::c_uint = ((boundary as libc::c_long
    - (*archive_handle).offset % boundary as libc::c_long)
    % boundary as libc::c_long) as libc::c_uint;
  (*archive_handle).seek.expect("non-null function pointer")(
    (*archive_handle).src_fd,
    skip_amount as off_t,
  );
  (*archive_handle).offset += skip_amount as libc::c_long;
}
