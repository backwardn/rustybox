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





extern "C" {
  #[no_mangle]
  fn remove_file(path: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
}

pub type C2RustUnnamed = libc::c_int;
pub const FILEUTILS_IGNORE_CHMOD_ERR: C2RustUnnamed = -2147483648;
pub const FILEUTILS_REFLINK_ALWAYS: C2RustUnnamed = 262144;
pub const FILEUTILS_REFLINK: C2RustUnnamed = 131072;
pub const FILEUTILS_RMDEST: C2RustUnnamed = 32768;
pub const FILEUTILS_NO_TARGET_DIR: C2RustUnnamed = 16384;
pub const FILEUTILS_UPDATE: C2RustUnnamed = 8192;
pub const FILEUTILS_VERBOSE: C2RustUnnamed = 4096;
pub const FILEUTILS_DEREFERENCE_L0: C2RustUnnamed = 256;
pub const FILEUTILS_DEREF_SOFTLINK: C2RustUnnamed = 128;
pub const FILEUTILS_MAKE_SOFTLINK: C2RustUnnamed = 64;
pub const FILEUTILS_MAKE_HARDLINK: C2RustUnnamed = 32;
pub const FILEUTILS_INTERACTIVE: C2RustUnnamed = 16;
pub const FILEUTILS_FORCE: C2RustUnnamed = 8;
pub const FILEUTILS_RECUR: C2RustUnnamed = 4;
pub const FILEUTILS_DEREFERENCE: C2RustUnnamed = 2;
pub const FILEUTILS_PRESERVE_STATUS: C2RustUnnamed = 1;

/*
 * Copyright (c) 2017 Denys Vlasenko <vda.linux@googlemail.com>
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
//config:config NUKE
//config:	bool "nuke (2.9 kb)"
//config:	default y
//config:	help
//config:	Alias to "rm -rf".
//applet:IF_NUKE(APPLET_NOEXEC(nuke, nuke, BB_DIR_BIN, BB_SUID_DROP, nuke))
//kbuild:lib-$(CONFIG_NUKE) += nuke.o
//usage:#define nuke_trivial_usage
//usage:       "DIR..."
//usage:#define nuke_full_usage "\n\n"
//usage:       "Remove DIRs"
/* This is a NOEXEC applet. Be very careful! */
#[no_mangle]
pub unsafe extern "C" fn nuke_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  loop
  // klibc-utils do not check opts, will try to delete "-dir" args
  //opt = getopt32(argv, "");
  //argv += optind;
  {
    argv = argv.offset(1);
    if (*argv).is_null() {
      break;
    }
    remove_file(
      *argv,
      FILEUTILS_FORCE as libc::c_int | FILEUTILS_RECUR as libc::c_int,
    );
  }
  // klibc-utils do not indicate errors
  return 0i32;
}
