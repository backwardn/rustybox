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
  fn xopen(pathname: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
  #[no_mangle]
  fn single_argv(argv: *mut *mut libc::c_char) -> *mut libc::c_char;
  #[no_mangle]
  fn bb_xioctl(
    fd: libc::c_int,
    request: libc::c_uint,
    argp: *mut libc::c_void,
    ioctl_name: *const libc::c_char,
  ) -> libc::c_int;
}

/*
 * raidautorun implementation for busybox
 *
 * Copyright (C) 2006 Bernhard Reutner-Fischer
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
//config:config RAIDAUTORUN
//config:	bool "raidautorun (1.3 kb)"
//config:	default y
//config:	select PLATFORM_LINUX
//config:	help
//config:	raidautorun tells the kernel md driver to
//config:	search and start RAID arrays.
//applet:IF_RAIDAUTORUN(APPLET_NOEXEC(raidautorun, raidautorun, BB_DIR_SBIN, BB_SUID_DROP, raidautorun))
//kbuild:lib-$(CONFIG_RAIDAUTORUN) += raidautorun.o
//usage:#define raidautorun_trivial_usage
//usage:       "DEVICE"
//usage:#define raidautorun_full_usage "\n\n"
//usage:       "Tell the kernel to automatically search and start RAID arrays"
//usage:
//usage:#define raidautorun_example_usage
//usage:       "$ raidautorun /dev/md0"
#[no_mangle]
pub unsafe extern "C" fn raidautorun_main(
  mut _argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  bb_xioctl(
    xopen(single_argv(argv), 0),
    0u32 << 0 + 8 + 8 + 14
      | (9 << 0 + 8) as libc::c_uint
      | (0x14 << 0) as libc::c_uint
      | (0 << 0 + 8 + 8) as libc::c_uint,
    0 as *mut libc::c_void,
    b"RAID_AUTORUN\x00" as *const u8 as *const libc::c_char,
  );
  return 0;
}
