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
  fn bb_show_usage() -> !;
  #[no_mangle]
  fn bb_perror_nomsg_and_die() -> !;

  /*
   * pivot_root.c - Change root file system.  Based on util-linux 2.10s
   *
   * busyboxed by Evin Robertson
   * pivot_root syscall stubbed by Erik Andersen, so it will compile
   *     regardless of the kernel being used.
   *
   * Licensed under GPLv2, see file LICENSE in this source tree.
   */
  //config:config PIVOT_ROOT
  //config:	bool "pivot_root (1.1 kb)"
  //config:	default y
  //config:	select PLATFORM_LINUX
  //config:	help
  //config:	The pivot_root utility swaps the mount points for the root filesystem
  //config:	with some other mounted filesystem. This allows you to do all sorts
  //config:	of wild and crazy things with your Linux system and is far more
  //config:	powerful than 'chroot'.
  //config:
  //config:	Note: This is for initrd in linux 2.4. Under initramfs (introduced
  //config:	in linux 2.6) use switch_root instead.
  //applet:IF_PIVOT_ROOT(APPLET_NOFORK(pivot_root, pivot_root, BB_DIR_SBIN, BB_SUID_DROP, pivot_root))
  //kbuild:lib-$(CONFIG_PIVOT_ROOT) += pivot_root.o
  //usage:#define pivot_root_trivial_usage
  //usage:       "NEW_ROOT PUT_OLD"
  //usage:#define pivot_root_full_usage "\n\n"
  //usage:       "Move the current root file system to PUT_OLD and make NEW_ROOT\n"
  //usage:       "the new root file system"
  #[no_mangle]
  fn pivot_root(new_root: *const libc::c_char, put_old: *const libc::c_char) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn pivot_root_main(
  mut argc: libc::c_int,
  mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
  if argc != 3i32 {
    bb_show_usage();
  }
  /* NOFORK applet. Hardly matters wrt performance, but code is trivial */
  if pivot_root(*argv.offset(1), *argv.offset(2)) < 0i32 {
    /* prints "pivot_root: <strerror text>" */
    bb_perror_nomsg_and_die();
  }
  return 0i32;
}
