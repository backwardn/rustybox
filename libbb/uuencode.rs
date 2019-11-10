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
  fn getc_unlocked(__stream: *mut FILE) -> libc::c_int;
  #[no_mangle]
  fn fwrite(__ptr: *const libc::c_void, __size: size_t, __n: size_t, __s: *mut FILE) -> size_t;
  #[no_mangle]
  fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;


  #[no_mangle]
  fn strlen(__s: *const libc::c_char) -> size_t;
  #[no_mangle]
  fn bb_simple_error_msg_and_die(s: *const libc::c_char) -> !;
}

use crate::librb::size_t;

use libc::FILE;
pub type C2RustUnnamed = libc::c_uint;
pub const BASE64_FLAG_NO_STOP_CHAR: C2RustUnnamed = 128;
pub const BASE64_FLAG_UU_STOP: C2RustUnnamed = 256;
pub const BUFFER_SIZE: C2RustUnnamed_0 = 64;
pub type C2RustUnnamed_0 = libc::c_uint;

/*
 * Copyright 2003, Glenn McGrath
 * Copyright 2006, Rob Landley <rob@landley.net>
 * Copyright 2010, Denys Vlasenko
 *
 * Licensed under GPLv2 or later, see file LICENSE in this source tree.
 */
/* Conversion table.  for base 64 */
#[no_mangle]
pub static mut bb_uuenc_tbl_base64: [libc::c_char; 66] = [
  'A' as i32 as libc::c_char,
  'B' as i32 as libc::c_char,
  'C' as i32 as libc::c_char,
  'D' as i32 as libc::c_char,
  'E' as i32 as libc::c_char,
  'F' as i32 as libc::c_char,
  'G' as i32 as libc::c_char,
  'H' as i32 as libc::c_char,
  'I' as i32 as libc::c_char,
  'J' as i32 as libc::c_char,
  'K' as i32 as libc::c_char,
  'L' as i32 as libc::c_char,
  'M' as i32 as libc::c_char,
  'N' as i32 as libc::c_char,
  'O' as i32 as libc::c_char,
  'P' as i32 as libc::c_char,
  'Q' as i32 as libc::c_char,
  'R' as i32 as libc::c_char,
  'S' as i32 as libc::c_char,
  'T' as i32 as libc::c_char,
  'U' as i32 as libc::c_char,
  'V' as i32 as libc::c_char,
  'W' as i32 as libc::c_char,
  'X' as i32 as libc::c_char,
  'Y' as i32 as libc::c_char,
  'Z' as i32 as libc::c_char,
  'a' as i32 as libc::c_char,
  'b' as i32 as libc::c_char,
  'c' as i32 as libc::c_char,
  'd' as i32 as libc::c_char,
  'e' as i32 as libc::c_char,
  'f' as i32 as libc::c_char,
  'g' as i32 as libc::c_char,
  'h' as i32 as libc::c_char,
  'i' as i32 as libc::c_char,
  'j' as i32 as libc::c_char,
  'k' as i32 as libc::c_char,
  'l' as i32 as libc::c_char,
  'm' as i32 as libc::c_char,
  'n' as i32 as libc::c_char,
  'o' as i32 as libc::c_char,
  'p' as i32 as libc::c_char,
  'q' as i32 as libc::c_char,
  'r' as i32 as libc::c_char,
  's' as i32 as libc::c_char,
  't' as i32 as libc::c_char,
  'u' as i32 as libc::c_char,
  'v' as i32 as libc::c_char,
  'w' as i32 as libc::c_char,
  'x' as i32 as libc::c_char,
  'y' as i32 as libc::c_char,
  'z' as i32 as libc::c_char,
  '0' as i32 as libc::c_char,
  '1' as i32 as libc::c_char,
  '2' as i32 as libc::c_char,
  '3' as i32 as libc::c_char,
  '4' as i32 as libc::c_char,
  '5' as i32 as libc::c_char,
  '6' as i32 as libc::c_char,
  '7' as i32 as libc::c_char,
  '8' as i32 as libc::c_char,
  '9' as i32 as libc::c_char,
  '+' as i32 as libc::c_char,
  '/' as i32 as libc::c_char,
  '=' as i32 as libc::c_char,
  '\u{0}' as i32 as libc::c_char,
];
#[no_mangle]
pub static mut bb_uuenc_tbl_std: [libc::c_char; 65] = [
  '`' as i32 as libc::c_char,
  '!' as i32 as libc::c_char,
  '\"' as i32 as libc::c_char,
  '#' as i32 as libc::c_char,
  '$' as i32 as libc::c_char,
  '%' as i32 as libc::c_char,
  '&' as i32 as libc::c_char,
  '\'' as i32 as libc::c_char,
  '(' as i32 as libc::c_char,
  ')' as i32 as libc::c_char,
  '*' as i32 as libc::c_char,
  '+' as i32 as libc::c_char,
  ',' as i32 as libc::c_char,
  '-' as i32 as libc::c_char,
  '.' as i32 as libc::c_char,
  '/' as i32 as libc::c_char,
  '0' as i32 as libc::c_char,
  '1' as i32 as libc::c_char,
  '2' as i32 as libc::c_char,
  '3' as i32 as libc::c_char,
  '4' as i32 as libc::c_char,
  '5' as i32 as libc::c_char,
  '6' as i32 as libc::c_char,
  '7' as i32 as libc::c_char,
  '8' as i32 as libc::c_char,
  '9' as i32 as libc::c_char,
  ':' as i32 as libc::c_char,
  ';' as i32 as libc::c_char,
  '<' as i32 as libc::c_char,
  '=' as i32 as libc::c_char,
  '>' as i32 as libc::c_char,
  '?' as i32 as libc::c_char,
  '@' as i32 as libc::c_char,
  'A' as i32 as libc::c_char,
  'B' as i32 as libc::c_char,
  'C' as i32 as libc::c_char,
  'D' as i32 as libc::c_char,
  'E' as i32 as libc::c_char,
  'F' as i32 as libc::c_char,
  'G' as i32 as libc::c_char,
  'H' as i32 as libc::c_char,
  'I' as i32 as libc::c_char,
  'J' as i32 as libc::c_char,
  'K' as i32 as libc::c_char,
  'L' as i32 as libc::c_char,
  'M' as i32 as libc::c_char,
  'N' as i32 as libc::c_char,
  'O' as i32 as libc::c_char,
  'P' as i32 as libc::c_char,
  'Q' as i32 as libc::c_char,
  'R' as i32 as libc::c_char,
  'S' as i32 as libc::c_char,
  'T' as i32 as libc::c_char,
  'U' as i32 as libc::c_char,
  'V' as i32 as libc::c_char,
  'W' as i32 as libc::c_char,
  'X' as i32 as libc::c_char,
  'Y' as i32 as libc::c_char,
  'Z' as i32 as libc::c_char,
  '[' as i32 as libc::c_char,
  '\\' as i32 as libc::c_char,
  ']' as i32 as libc::c_char,
  '^' as i32 as libc::c_char,
  '_' as i32 as libc::c_char,
  '`' as i32 as libc::c_char,
];
/*
 * Encode bytes at S of length LENGTH to uuencode or base64 format and place it
 * to STORE.  STORE will be 0-terminated, and must point to a writable
 * buffer of at least 1+BASE64_LENGTH(length) bytes.
 * where BASE64_LENGTH(len) = (4 * ((LENGTH + 2) / 3))
 */
#[no_mangle]
pub unsafe extern "C" fn bb_uuencode(
  mut p: *mut libc::c_char,
  mut src: *const libc::c_void,
  mut length: libc::c_int,
  mut tbl: *const libc::c_char,
) {
  let mut s: *const libc::c_uchar = src as *const libc::c_uchar;
  /* Transform the 3x8 bits to 4x6 bits */
  while length > 0i32 {
    let mut s1: libc::c_uint = 0;
    let mut s2: libc::c_uint = 0;
    /* Are s[1], s[2] valid or should be assumed 0? */
    s2 = 0i32 as libc::c_uint; /* can be >=0, -1, -2 */
    s1 = s2;
    length -= 3i32;
    if length >= -1i32 {
      s1 = *s.offset(1) as libc::c_uint;
      if length >= 0i32 {
        s2 = *s.offset(2) as libc::c_uint
      }
    }
    let fresh0 = p;
    p = p.offset(1);
    *fresh0 = *tbl.offset((*s.offset(0) as libc::c_int >> 2i32) as isize);
    let fresh1 = p;
    p = p.offset(1);
    *fresh1 = *tbl.offset(
      (((*s.offset(0) as libc::c_int & 3i32) << 4i32) as libc::c_uint).wrapping_add(s1 >> 4i32)
        as isize,
    );
    let fresh2 = p;
    p = p.offset(1);
    *fresh2 =
      *tbl.offset(((s1 & 0xfi32 as libc::c_uint) << 2i32).wrapping_add(s2 >> 6i32) as isize);
    let fresh3 = p;
    p = p.offset(1);
    *fresh3 = *tbl.offset((s2 & 0x3fi32 as libc::c_uint) as isize);
    s = s.offset(3)
  }
  /* Zero-terminate */
  *p = '\u{0}' as i32 as libc::c_char;
  /* If length is -2 or -1, pad last char or two */
  while length != 0 {
    p = p.offset(-1);
    *p = *tbl.offset(64);
    length += 1
  }
}
/*
 * Decode base64 encoded string. Stops on '\0'.
 *
 * Returns: pointer to the undecoded part of source.
 * If points to '\0', then the source was fully decoded.
 * (*pp_dst): advanced past the last written byte.
 */
#[no_mangle]
pub unsafe extern "C" fn decode_base64(
  mut pp_dst: *mut *mut libc::c_char,
  mut src: *const libc::c_char,
) -> *const libc::c_char {
  let mut dst: *mut libc::c_char = *pp_dst; /* while (1) */
  let mut src_tail: *const libc::c_char = 0 as *const libc::c_char;
  's_11: loop {
    let mut six_bit: [libc::c_uchar; 4] = [0; 4];
    let mut count: libc::c_int = 0i32;
    /* Note that if we decode "AA==" and ate first '=',
     * we just decoded one char (count == 2) and now we'll
     * do the loop once more to decode second '='.
     */
    /* Fetch up to four 6-bit values */
    src_tail = src;
    while count < 4i32 {
      let mut table_ptr: *mut libc::c_char = 0 as *mut libc::c_char;
      let mut ch: libc::c_int = 0;
      /* Get next _valid_ character.
       * bb_uuenc_tbl_base64[] contains this string:
       *  0         1         2         3         4         5         6
       *  01234567890123456789012345678901234567890123456789012345678901234
       * "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/="
       */
      loop {
        ch = *src as libc::c_int;
        if ch == '\u{0}' as i32 {
          if count == 0i32 {
            //TODO: add BASE64_FLAG_foo to die on bad char?
            /* Example:
             * If we decode "QUJD <NUL>", we want
             * to return ptr to NUL, not to ' ',
             * because we did fully decode
             * the string (to "ABC").
             */
            src_tail = src
          }
          break 's_11;
        } else {
          src = src.offset(1);
          table_ptr = strchr(bb_uuenc_tbl_base64.as_ptr(), ch);
          if !table_ptr.is_null() {
            break;
          }
        }
      }
      /* Convert encoded character to decimal */
      ch =
        table_ptr.wrapping_offset_from(bb_uuenc_tbl_base64.as_ptr()) as libc::c_long as libc::c_int;
      /* ch is 64 if char was '=', otherwise 0..63 */
      if ch == 64i32 {
        break;
      }
      six_bit[count as usize] = ch as libc::c_uchar;
      count += 1
    }
    /* Transform 6-bit values to 8-bit ones.
     * count can be < 4 when we decode the tail:
     * "eQ==" -> "y", not "y NUL NUL".
     * Note that (count > 1) is always true,
     * "x===" encoding is not valid:
     * even a single zero byte encodes as "AA==".
     * However, with current logic we come here with count == 1
     * when we decode "==" tail.
     */
    if count > 1i32 {
      let fresh4 = dst;
      dst = dst.offset(1);
      *fresh4 =
        ((six_bit[0] as libc::c_int) << 2i32 | six_bit[1] as libc::c_int >> 4i32) as libc::c_char
    }
    if count > 2i32 {
      let fresh5 = dst;
      dst = dst.offset(1);
      *fresh5 =
        ((six_bit[1] as libc::c_int) << 4i32 | six_bit[2] as libc::c_int >> 2i32) as libc::c_char
    }
    if count > 3i32 {
      let fresh6 = dst;
      dst = dst.offset(1);
      *fresh6 = ((six_bit[2] as libc::c_int) << 6i32 | six_bit[3] as libc::c_int) as libc::c_char
    }
  }
  *pp_dst = dst;
  return src_tail;
}

/*
 * Busybox main internal header file
 *
 * Based in part on code from sash, Copyright (c) 1999 by David I. Bell
 * Permission has been granted to redistribute this code under GPL.
 *
 * Licensed under GPLv2, see file LICENSE in this source tree.
 */
/* TODO: and glibc? */
/* There are two incompatible basename's, let's not use them! */
/* See the dirname/basename man page for details */
/* dirname,basename */
/* Don't do this here:
 * #include <sys/sysinfo.h>
 * Some linux/ includes pull in conflicting definition
 * of struct sysinfo (only in some toolchanins), which breaks build.
 * Include sys/sysinfo.h only in those files which need it.
 */
/* Just in case libc doesn't define some of these... */
//This breaks on bionic:
//# if !defined(__socklen_t_defined) && !defined(_SOCKLEN_T_DECLARED)
// /* We #define socklen_t *after* includes, otherwise we get
// * typedef redefinition errors from system headers
// * (in case "is it defined already" detection above failed)
// */
//#  define socklen_t bb_socklen_t
//   typedef unsigned socklen_t;
//# endif
//if this is still needed, add a fix along the lines of
//  ifdef SPECIFIC_BROKEN_LIBC_CHECK / typedef socklen_t / endif
//in platform.h instead!
/*
 * Use '%m' to append error string on platforms that support it,
 * '%s' and strerror() on those that don't.
 */
/*nothing*/
/* Some libc's forget to declare these, do it ourself */
/* klogctl is in libc's klog.h, but we cheat and not #include that */
/* Busybox does not use threads, we can speed up stdio. */
/* Above functions are required by POSIX.1-2008, below ones are extensions */
/* musl <= 1.1.15 does not support fflush_unlocked(NULL) */
//# undef  fflush
//# define fflush(stream) fflush_unlocked(stream)
/* Make all declarations hidden (-fvisibility flag only affects definitions) */
/* (don't include system headers after this until corresponding pop!) */
/* Tested to work correctly with all int types (IIRC :]) */
/* Large file support */
/* Note that CONFIG_LFS=y forces bbox to be built with all common ops
 * (stat, lseek etc) mapped to "largefile" variants by libc.
 * Practically it means that open() automatically has O_LARGEFILE added
 * and all filesize/file_offset parameters and struct members are "large"
 * (in today's world - signed 64bit). For full support of large files,
 * we need a few helper #defines (below) and careful use of off_t
 * instead of int/ssize_t. No lseek64(), O_LARGEFILE etc necessary */
/* CONFIG_LFS is on */
/* "long" is long enough on this system */
/* usage: sz = BB_STRTOOFF(s, NULL, 10); if (errno || sz < 0) die(); */
/* usage: printf("size: %"OFF_FMT"d (%"OFF_FMT"x)\n", sz, sz); */
/* scary. better ideas? (but do *test* them first!) */
/* Users report bionic to use 32-bit off_t even if LARGEFILE support is requested.
 * We misdetected that. Don't let it build:
 */
/* Some useful definitions */
/* Macros for min/max.  */
/* buffer allocation schemes */
/* glibc uses __errno_location() to get a ptr to errno */
/* We can just memorize it once - no multithreading in busybox :) */
/* dmalloc will redefine these to it's own implementation. It is safe
 * to have the prototypes here unconditionally.  */
/* After v = xrealloc_vector(v, SHIFT, idx) it's ok to use
 * at least v[idx] and v[idx+1], for all idx values.
 * SHIFT specifies how many new elements are added (1:2, 2:4, ..., 8:256...)
 * when all elements are used up. New elements are zeroed out.
 * xrealloc_vector(v, SHIFT, idx) *MUST* be called with consecutive IDXs -
 * skipping an index is a bad bug - it may miss a realloc!
 */
//TODO: supply a pointer to char[11] buffer (avoid statics)?
/* cp.c, mv.c, install.c depend on these values. CAREFUL when changing them! */
/* -p */
/* !-d */
/* -R */
/* -f */
/* -i */
/* -l */
/* -s */
/* -L */
/* -H */
/* -a = -pdR (mapped in cp.c) */
/* -r = -dR  (mapped in cp.c) */
/* -P = -d   (mapped in cp.c) */
/* -v */
/* -u */
/* -T */
/* --remove-destination */
/* bit 17 skipped for "cp --parents" */
/* cp --reflink=auto */
/* cp --reflink[=always] */
/*
 * Hole. cp may have some bits set here,
 * they should not affect remove_file()/copy_file()
 */
/* NB: without FILEUTILS_RECUR in flags, it will basically "cat"
 * the source, not copy (unless "source" is a directory).
 * This makes "cp /dev/null file" and "install /dev/null file" (!!!)
 * work coreutils-compatibly. */
/*ACTION_REVERSE      = (1 << 4), - unused */
/* more than enough for "/dev/ttyXXX" */
/* bb_copyfd_XX print read/write errors and return -1 if they occur */
/* "short" copy can be detected by return value < size */
/* this helper yells "short read!" if param is not -1 */
/* xxxx_strip version can modify its parameter:
 * "/"        -> "/"
 * "abc"      -> "abc"
 * "abc/def"  -> "def"
 * "abc/def/" -> "def" !!
 */
/* "abc/def/" -> "" and it never modifies 'path' */
/* Simpler version: does not special case "/" string */
/* NB: can violate const-ness (similarly to strchr) */
/* !RETURNS_MALLOC: it's a realloc-like function */
/* bb_signals(BB_FATAL_SIGS, handler) catches all signals which
 * otherwise would kill us, except for those resulting from bugs:
 * SIGSEGV, SIGILL, SIGFPE.
 * Other fatal signals not included (TODO?):
 * SIGBUS   Bus error (bad memory access)
 * SIGPOLL  Pollable event. Synonym of SIGIO
 * SIGPROF  Profiling timer expired
 * SIGSYS   Bad argument to routine
 * SIGTRAP  Trace/breakpoint trap
 *
 * The only known arch with some of these sigs not fitting
 * into 32 bits is parisc (SIGXCPU=33, SIGXFSZ=34, SIGSTKFLT=36).
 * Dance around with long long to guard against that...
 */
// Write to pipe with no readers
// Quit from keyboard
// Abort signal from abort(3)
// Timer signal from alarm(2)
// Virtual alarm clock
// CPU time limit exceeded
// File size limit exceeded
// Yes kids, these are also fatal!
/* Unlike signal() and bb_signals, sets handler with sigaction()
 * and in a way that while signal handler is run, no other signals
 * will be blocked; syscalls will not be restarted: */
/* syscalls like read() will be interrupted with EINTR: */
/* syscalls like read() won't be interrupted (though select/poll will be): */
/* Will do sigaction(signum, act, NULL): */
/* SIG_BLOCK/SIG_UNBLOCK all signals: */
/* Return old set in the same set: */
/* Standard handler which just records signo */
/* not FAST_FUNC! */
/* In this form code with pipes is much more readable */
/* Useful for having small structure members/global variables */
/* | AF_DECnet */
/* | AF_IPX */
/* SO_REUSEADDR allows a server to rebind to an address that is already
 * "in use" by old connections to e.g. previous server instance which is
 * killed or crashed. Without it bind will fail until all such connections
 * time out. Linux does not allow multiple live binds on same ip:port
 * regardless of SO_REUSEADDR (unlike some other flavors of Unix).
 * Turn it on before you call bind(). */
/* On Linux this never fails. */
/* NB: returns port in host byte order */
/* Create stream socket, and allocate suitable lsa.
 * (lsa of correct size and lsa->sa.sa_family (AF_INET/AF_INET6))
 * af == AF_UNSPEC will result in trying to create IPv6 socket,
 * and if kernel doesn't support it, fall back to IPv4.
 * This is useful if you plan to bind to resulting local lsa.
 */
/* Create server socket bound to bindaddr:port. bindaddr can be NULL,
 * numeric IP ("N.N.N.N") or numeric IPv6 address,
 * and can have ":PORT" suffix (for IPv6 use "[X:X:...:X]:PORT").
 * Only if there is no suffix, port argument is used */
/* NB: these set SO_REUSEADDR before bind */
/* Create client TCP socket connected to peer:port. Peer cannot be NULL.
 * Peer can be numeric IP ("N.N.N.N"), numeric IPv6 address or hostname,
 * and can have ":PORT" suffix (for IPv6 use "[X:X:...:X]:PORT").
 * If there is no suffix, port argument is used */
/* Connect to peer identified by lsa */
/* Get local address of bound or accepted socket */
/* Get remote address of connected or accepted socket */
/* Return malloc'ed len_and_sockaddr with socket address of host:port
 * Currently will return IPv4 or IPv6 sockaddrs only
 * (depending on host), but in theory nothing prevents e.g.
 * UNIX socket address being returned, IPX sockaddr etc...
 * On error does bb_error_msg and returns NULL */
/* Version which dies on error */
/* Same, useful if you want to force family (e.g. IPv6) */
/* Assign sin[6]_port member if the socket is an AF_INET[6] one,
 * otherwise no-op. Useful for ftp.
 * NB: does NOT do htons() internally, just direct assignment. */
/* Retrieve sin[6]_port or return -1 for non-INET[6] lsa's */
/* Reverse DNS. Returns NULL on failure. */
/* This one doesn't append :PORTNUM */
/* This one also doesn't fall back to dotted IP (returns NULL) */
/* inet_[ap]ton on steroids */
// "old" (ipv4 only) API
// users: traceroute.c hostname.c - use _list_ of all IPs
// Also mount.c and inetd.c are using gethostbyname(),
// + inet_common.c has additional IPv4-only stuff
/* opaque */
// RFC 5246
// sequence number
//   Each connection state contains a sequence number, which is
//   maintained separately for read and write states.  The sequence
//   number MUST be set to zero whenever a connection state is made the
//   active state.  Sequence numbers are of type uint64 and may not
//   exceed 2^64-1.
/*uint64_t read_seq64_be;*/
/*u8 *server_write_MAC_key;*/
//used by AES_GCM
/* 0 if argv[0] is NULL: */
/* Guaranteed to NOT be a macro (smallest code). Saves nearly 2k on uclibc.
 * But potentially slow, don't use in one-billion-times loops */
/* Note: does not use stdio, writes to fd 2 directly */
// gcc-4.1.1 still isn't good enough at optimizing it
// (+200 bytes compared to macro)
//static ALWAYS_INLINE
//int LONE_DASH(const char *s) { return s[0] == '-' && !s[1]; }
//static ALWAYS_INLINE
//int NOT_LONE_DASH(const char *s) { return s[0] != '-' || s[1]; }
/* Returns a string with unprintable chars replaced by '?' or
 * SUBST_WCHAR. This function is unicode-aware. */
/* Prints unprintable char ch as ^C or M-c to file
 * (M-c is used only if ch is ORed with PRINTABLE_META),
 * else it is printed as-is (except for ch = 0x9b) */
/* Return a string that is the printable representation of character ch.
 * Buffer must hold at least four characters. */
// NB: will return short read on error, not -1,
// if some data was read before error occurred
// Reads one line a-la fgets (but doesn't save terminating '\n').
// Reads byte-by-byte. Useful when it is important to not read ahead.
// Bytes are appended to pfx (which must be malloced, or NULL).
/* Reads block up to *maxsz_p (default: INT_MAX - 4095) */
/* Returns NULL if file can't be opened (default max size: INT_MAX - 4095) */
/* Never returns NULL */
/* Else use variable one (a bit more expensive) */
/* Autodetects gzip/bzip2 formats. fd may be in the middle of the file! */
/* Autodetects .gz etc */
/* lzma has no signature, need a little helper. NB: exist only for ENABLE_FEATURE_SEAMLESS_LZMA=y */
// NB: will return short write on error, not -1,
// if some data was written before error occurred
/* Close fd, but check for failures (some types of write errors) */
/* Reads and prints to stdout till eof, then closes FILE. Exits on error: */
/* Reads a line from a text file, up to a newline or NUL byte, inclusive.
 * Returns malloc'ed char*. If end is NULL '\n' isn't considered
 * end of line. If end isn't NULL, length of the chunk is stored in it.
 * Returns NULL if EOF/error.
 */
/* Reads up to (and including) TERMINATING_STRING: */
/* Same, with limited max size, and returns the length (excluding NUL): */
/* Chops off TERMINATING_STRING from the end: */
/* Reads up to (and including) "\n" or NUL byte: */
/* Chops off '\n' from the end, unlike fgets: */
/* Same, but doesn't try to conserve space (may have some slack after the end) */
/* extern char *xmalloc_fgetline_fast(FILE *file) FAST_FUNC RETURNS_MALLOC; */
/* Prints warning to stderr and returns NULL on failure: */
/* "Opens" stdin if filename is special, else just opens file: */
/* not FAST_FUNC! */
/* Wrapper which restarts poll on EINTR or ENOMEM.
 * On other errors complains [perror("poll")] and returns.
 * Warning! May take (much) longer than timeout_ms to return!
 * If this is a problem, use bare poll and open-code EINTR/ENOMEM handling */
/* Convert each alpha char in str to lower-case */
/* Returns a pointer past the formatted number, does NOT null-terminate */
/* Intelligent formatters of bignums */
/* If block_size == 0, display size without fractional part,
 * else display (size * block_size) with one decimal digit.
 * If display_unit == 0, show value no bigger than 1024 with suffix (K,M,G...),
 * else divide by display_unit and do not use suffix. */
/* "1024.0G" */
//TODO: provide pointer to buf (avoid statics)?
/* Put a string of hex bytes ("1b2e66fe"...), return advanced pointer */
/* Reverse */
/* Generate a UUID */
/* Last element is marked by mult == 0 */
/* Specialized: */
/* Using xatoi() instead of naive atoi() is not always convenient -
 * in many places people want *non-negative* values, but store them
 * in signed int. Therefore we need this one:
 * dies if input is not in [0, INT_MAX] range. Also will reject '-0' etc.
 * It should really be named xatoi_nonnegative (since it allows 0),
 * but that would be too long.
 */
/* Useful for reading port numbers */
/* These parse entries in /etc/passwd and /etc/group.  This is desirable
 * for BusyBox since we want to avoid using the glibc NSS stuff, which
 * increases target size and is often not needed on embedded systems.  */
/* wrapper: allows string to contain numeric uid or gid */
/* always sets uid and gid; returns 0 on failure */
/* always sets uid and gid; exits on failure */
/* chown-like handling of "user[:[group]" */
/* versions which cache results (useful for ps, ls etc) */
/* internally usernames are saved in fixed-sized char[] buffers */
/*
 * Returns (-1) terminated malloced result of getgroups().
 * Reallocs group_array (useful for repeated calls).
 * ngroups is an initial size of array. It is rounded up to 32 for realloc.
 * ngroups is updated on return.
 * ngroups can be NULL: bb_getgroups(NULL, NULL) is valid usage.
 * Dies on errors (on Linux, only xrealloc can cause this, not internal getgroups call).
 */
/* BB_EXECxx always execs (it's not doing NOFORK/NOEXEC stuff),
 * but it may exec busybox and call applet instead of searching PATH.
 */
/* xvfork() can't be a _function_, return after vfork in child mangles stack
 * in the parent. It must be a macro. */
/* NOMMU friendy fork+exec: */
/* wait4pid: unlike waitpid, waits ONLY for one process.
 * Returns sig + 0x180 if child is killed by signal.
 * It's safe to pass negative 'pids' from failed [v]fork -
 * wait4pid will return -1 (and will not clobber [v]fork's errno).
 * IOW: rc = wait4pid(spawn(argv));
 *      if (rc < 0) bb_perror_msg("%s", argv[0]);
 *      if (rc > 0) bb_error_msg("exit code: %d", rc & 0xff);
 */
/* ***********************************************************************/
/* spawn_and_wait/run_nofork_applet/run_applet_no_and_exit need to work */
/* carefully together to reinit some global state while not disturbing  */
/* other. Be careful if you change them. Consult docs/nofork_noexec.txt */
/* ***********************************************************************/
/* Same as wait4pid(spawn(argv)), but with NOFORK/NOEXEC if configured: */
/* Does NOT check that applet is NOFORK, just blindly runs it */
/* Helpers for daemonization.
 *
 * bb_daemonize(flags) = daemonize, does not compile on NOMMU
 *
 * bb_daemonize_or_rexec(flags, argv) = daemonizes on MMU (and ignores argv),
 *      rexec's itself on NOMMU with argv passed as command line.
 * Thus bb_daemonize_or_rexec may cause your <applet>_main() to be re-executed
 * from the start. (It will detect it and not reexec again second time).
 * You have to audit carefully that you don't do something twice as a result
 * (opening files/sockets, parsing config files etc...)!
 *
 * Both of the above will redirect fd 0,1,2 to /dev/null and drop ctty
 * (will do setsid()).
 *
 * fork_or_rexec(argv) = bare-bones fork on MMU,
 *      "vfork + re-exec ourself" on NOMMU. No fd redirection, no setsid().
 *      On MMU ignores argv.
 *
 * Helper for network daemons in foreground mode:
 *
 * bb_sanitize_stdio() = make sure that fd 0,1,2 are opened by opening them
 * to /dev/null if they are not.
 */
/* internal use */
//DAEMON_DOUBLE_FORK     = 1 << 4, /* double fork to avoid controlling tty */
/* Clear dangerous stuff, set PATH. Return 1 if was run by different user. */
/* For top, ps. Some argv[i] are replaced by malloced "-opt" strings */
/* { "-", NULL } */
/* BSD-derived getopt() functions require that optind be set to 1 in
 * order to reset getopt() state.  This used to be generally accepted
 * way of resetting getopt().  However, glibc's getopt()
 * has additional getopt() state beyond optind (specifically, glibc
 * extensions such as '+' and '-' at the start of the string), and requires
 * that optind be set to zero to reset its state.  BSD-derived versions
 * of getopt() misbehaved if optind is set to 0 in order to reset getopt(),
 * and glibc's getopt() used to coredump if optind is set 1 in order
 * to reset getopt().
 * Then BSD introduced additional variable "optreset" which should be
 * set to 1 in order to reset getopt().  Sigh.  Standards, anyone?
 *
 * By ~2008, OpenBSD 3.4 was changed to survive glibc-like optind = 0
 * (to interpret it as if optreset was set).
 */
/*def __GLIBC__*/
/* BSD style */
/* Having next pointer as a first member allows easy creation
 * of "llist-compatible" structs, and using llist_FOO functions
 * on them.
 */
/* BTW, surprisingly, changing API to
 *   llist_t *llist_add_to(llist_t *old_head, void *data)
 * etc does not result in smaller code... */
/* start_stop_daemon and udhcpc are special - they want
 * to create pidfiles regardless of FEATURE_PIDFILE */
/* True only if we created pidfile which is *file*, not /dev/null etc */
/* We need to export XXX_main from libbusybox
 * only if we build "individual" binaries
 */
/* Embedded script support */
/* Applets which are useful from another applets */
/* If shell needs them, they exist even if not enabled as applets */
/* Similar, but used by chgrp, not shell */
/* Used by ftpd */
/* Don't need IF_xxx() guard for these */
/* Networking */
/* This structure defines protocol families and their handlers. */
/*int type,*/
/* may modify src */
/* This structure defines hardware protocols and their handlers. */
/*
 * If *devname is not NULL, use that name, otherwise try to find free one,
 * malloc and return it in *devname.
 * return value is the opened fd to the loop device, or < on error
 */
/* These constants match linux/loop.h (without BB_ prefix): */
/* Returns malloced str */
/* Like bb_ask_noecho, but asks on stdin with no timeout.  */
/* Returns -1 if input is invalid. current_mode is a base for e.g. "u+rw" */
/*
 * Config file parser
 */
// treat consecutive delimiters as one
// trim leading and trailing delimiters
// TODO: COLLAPSE and TRIM seem to always go in pair
// last token takes entire remainder of the line
// die if < min tokens found
// keep a copy of current line
// comments are recognized even if they aren't the first char
// delim[0] and delim[1] are two different allowed comment chars
// (so far, delim[0] will only work as comment char for full-line comment)
// (IOW: it works as if PARSE_EOL_COMMENTS is not set. sysctl applet is okay with this)
// comments are recognized even if there is whitespace before
// ("line start><space><tab><space>#comment" is also comment, not only "line start>#comment")
// NORMAL is:
// * remove leading and trailing delimiters and collapse
//   multiple delimiters into one
// * warn and continue if less than mintokens delimiters found
// * grab everything into last token
// * comments are recognized even if they aren't the first char
/* delims[0] is a comment char (use '\0' to disable), the rest are token delimiters */
/* Concatenate path and filename to new allocated buffer.
 * Add "/" only as needed (no duplicate "//" are produced).
 * If path is NULL, it is assumed to be "/".
 * filename should not be NULL. */
/* Returns NULL on . and .. */
/* Returns ptr to NUL */
/* Returns $SHELL, getpwuid(getuid())->pw_shell, or DEFAULT_SHELL.
 * Note that getpwuid result might need xstrdup'ing
 * if there is a possibility of intervening getpwxxx() calls.
 */
/* Structures inside "struct caps" are Linux-specific and libcap-specific: */
/* setup_environment:
 * if chdir pw->pw_dir: ok: else if to_tmp == 1: goto /tmp else: goto / or die
 * if clear_env = 1: cd(pw->pw_dir), clear environment, then set
 *   TERM=(old value)
 *   USER=pw->pw_name, LOGNAME=pw->pw_name
 *   PATH=bb_default_[root_]path
 *   HOME=pw->pw_dir
 *   SHELL=shell
 * else if change_env = 1:
 *   if not root (if pw->pw_uid != 0):
 *     USER=pw->pw_name, LOGNAME=pw->pw_name
 *   HOME=pw->pw_dir
 *   SHELL=shell
 * else does nothing
 *
 * NB: CHANGEENV and CLEARENV use setenv() - this leaks memory!
 * If setup_environment() is used is vforked child, this leaks memory _in parent too_!
 */
/* Returns a malloced string */
/*
 * rnd is additional random input. New one is returned.
 * Useful if you call crypt_make_salt many times in a row:
 * rnd = crypt_make_salt(buf1, 4, 0);
 * rnd = crypt_make_salt(buf2, 4, rnd);
 * rnd = crypt_make_salt(buf3, 4, rnd);
 * (otherwise we risk having same salt generated)
 */
/*, int rnd*/
/* "$N$" + sha_salt_16_bytes + NUL */
/* Returns number of lines changed, or -1 on error */
/* NB: typically you want to pass fd 0, not 1. Think 'applet | grep something' */
/* NB: "unsigned request" is crucial! "int request" will break some arches! */
/* At least glibc has horrendously large inline for this, so wrap it */
/* "Keycodes" that report an escape sequence.
 * We use something which fits into signed char,
 * yet doesn't represent any valid Unicode character.
 * Also, -1 is reserved for error indication and we don't use it. */
/* Used only if Alt/Ctrl/Shifted */
/* Used only if Alted */
/* ^^^^^ Be sure that last defined value is small enough.
 * Current read_key() code allows going up to -32 (0xfff..fffe0).
 * This gives three upper bits in LSB to play with:
 * KEYCODE_foo values are 0xfff..fffXX, lowest XX bits are: scavvvvv,
 * s=0 if SHIFT, c=0 if CTRL, a=0 if ALT,
 * vvvvv bits are the same for same key regardless of "shift bits".
 */
//KEYCODE_SHIFT_...   = KEYCODE_...   & ~0x80,
/* 0xfff..fff00 */
/* How long is the longest ESC sequence we know?
 * We want it big enough to be able to contain
 * cursor position sequence "ESC [ 9999 ; 9999 R"
 */
/* Note: fd may be in blocking or non-blocking mode, both make sense.
 * For one, less uses non-blocking mode.
 * Only the first read syscall inside read_key may block indefinitely
 * (unless fd is in non-blocking mode),
 * subsequent reads will time out after a few milliseconds.
 * Return of -1 means EOF or error (errno == 0 on EOF).
 * buffer[0] is used as a counter of buffered chars and must be 0
 * on first call.
 * timeout:
 * -2: do not poll(-1) for input - read() it, return on EAGAIN at once
 * -1: poll(-1) (i.e. block even on NONBLOCKed fd)
 * >=0: poll() for TIMEOUT milliseconds, return -1/EAGAIN on timeout
 */
/* It's NOT just ENABLEd or disabled. It's a number: */
/* must never be <= 0 */
/* meaning of this field depends on FEATURE_EDITING_SAVE_ON_EXIT:
 * if !FEATURE_EDITING_SAVE_ON_EXIT: "how many lines are
 * in on-disk history"
 * if FEATURE_EDITING_SAVE_ON_EXIT: "how many in-memory lines are
 * also in on-disk history (and thus need to be skipped on save)"
 */
/*
 * maxsize must be >= 2.
 * Returns:
 * -1 on read errors or EOF, or on bare Ctrl-D,
 * 0  on ctrl-C (the line entered is still returned in 'command'),
 * >0 length of input string, including terminating '\n'
 */
/* synchronize with sizeof(task_struct.comm) in /usr/include/linux/sched.h */
// For mixed 32/64 userspace, 32-bit pmap still needs
// 64-bit field here to correctly show 64-bit processes:
// (strictly speaking, other fields need to be wider too,
// but they are in kbytes, not bytes, and they hold sizes,
// not start addresses, sizes tend to be less than 4 terabytes)
/* Fields are set to 0/NULL if failed to determine (or not requested) */
/* Everything below must contain no ptrs to malloc'ed data:
 * it is memset(0) for each process in procps_scan() */
/* we round it to kbytes */
/* basename of executable in exec(2), read from /proc/N/stat
 * (if executable is symlink or script, it is NOT replaced
 * by link target or interpreter name) */
/* user/group? - use passwd/group parsing functions */
/* flag bits for procps_scan(xx, flags) calls */
/* PSSCAN_CMD      = 1 << 6, - use read_cmdline instead */
/* NB: used by find_pid_by_name(). Any applet using it
 * needs to be mentioned here. */
//procps_status_t* alloc_procps_scan(void) FAST_FUNC;
/* Format cmdline (up to col chars) into char buf[size] */
/* Puts [comm] if cmdline is empty (-> process is a kernel thread) */
/* Use strict=1 if you process input from untrusted source:
 * it will return NULL on invalid %xx (bad hex chars)
 * and str + 1 if decoded char is / or NUL.
 * In non-strict mode, it always succeeds (returns str),
 * and also it additionally decoded '+' to space.
 */
/* Sign-extends to a value which never matches fgetc result: */
/*
 * Decode base64 encoded stream.
 * Can stop on EOF, specified char, or on uuencode-style "====" line:
 * flags argument controls it.
 */
#[no_mangle]
pub unsafe extern "C" fn read_base64(
  mut src_stream: *mut FILE,
  mut dst_stream: *mut FILE,
  mut flags: libc::c_int,
) {
  /* Note that EOF _can_ be passed as exit_char too */
  /* uuencoded files have 61 byte lines. Use 64 byte buffer
   * to process line at a time.
   */
  let mut in_buf: [libc::c_char; 66] = [0; 66];
  let mut out_buf: [libc::c_char; 50] = [0; 50];
  let mut out_tail: *mut libc::c_char = 0 as *mut libc::c_char;
  let mut in_tail: *const libc::c_char = 0 as *const libc::c_char;
  let mut term_seen: libc::c_int = 0i32;
  let mut in_count: libc::c_int = 0i32;
  loop {
    while in_count < BUFFER_SIZE as libc::c_int {
      let mut ch: libc::c_int = getc_unlocked(src_stream);
      if ch == flags as libc::c_schar as libc::c_int {
        if in_count == 0i32 {
          return;
        }
        term_seen = 1i32;
        break;
      } else if ch == -1i32 {
        term_seen = 1i32;
        break;
      } else {
        /* Prevent "====" line to be split: stop if we see '\n'.
         * We can also skip other whitespace and skirt the problem
         * of files with NULs by stopping on any control char or space:
         */
        if ch <= ' ' as i32 {
          break;
        }
        let fresh7 = in_count;
        in_count = in_count + 1;
        in_buf[fresh7 as usize] = ch as libc::c_char
      }
    }
    in_buf[in_count as usize] = '\u{0}' as i32 as libc::c_char;
    /* Did we encounter "====" line? */
    if flags & BASE64_FLAG_UU_STOP as libc::c_int != 0
      && strcmp(
        in_buf.as_mut_ptr(),
        b"====\x00" as *const u8 as *const libc::c_char,
      ) == 0i32
    {
      return;
    }
    out_tail = out_buf.as_mut_ptr();
    in_tail = decode_base64(&mut out_tail, in_buf.as_mut_ptr());
    fwrite(
      out_buf.as_mut_ptr() as *const libc::c_void,
      out_tail.wrapping_offset_from(out_buf.as_mut_ptr()) as libc::c_long as size_t,
      1i32 as size_t,
      dst_stream,
    );
    if term_seen != 0 {
      /* Did we consume ALL characters? */
      if *in_tail as libc::c_int == '\u{0}' as i32 {
        return;
      }
      /* No */
      bb_simple_error_msg_and_die(
        b"truncated base64 input\x00" as *const u8 as *const libc::c_char,
      );
    }
    /* It was partial decode */
    in_count = strlen(in_tail) as libc::c_int;
    memmove(
      in_buf.as_mut_ptr() as *mut libc::c_void,
      in_tail as *const libc::c_void,
      in_count as libc::c_ulong,
    );
  }
}
