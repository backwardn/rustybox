use libc;

extern "C" {
  #[no_mangle]
  fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;

  #[no_mangle]
  fn volume_id_get_buffer(id: *mut volume_id, off_0: uint64_t, len: size_t) -> *mut libc::c_void;

  #[no_mangle]
  fn volume_id_set_label_string(id: *mut volume_id, buf: *const uint8_t, count: size_t);

  #[no_mangle]
  fn volume_id_set_uuid(id: *mut volume_id, buf: *const uint8_t, format: uuid_format);
}




use crate::librb::uint8_t;
use crate::librb::uint32_t;
use crate::librb::uint64_t;
use crate::librb::size_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct volume_id {
  pub fd: libc::c_int,
  pub error: libc::c_int,
  pub sbbuf_len: size_t,
  pub seekbuf_len: size_t,
  pub sbbuf: *mut uint8_t,
  pub seekbuf: *mut uint8_t,
  pub seekbuf_off: uint64_t,
  pub label: [libc::c_char; 65],
  pub uuid: [libc::c_char; 37],
  pub type_0: *const libc::c_char,
}

pub type uuid_format = libc::c_uint;
// pub const UUID_DCE_STRING: uuid_format = 3;
pub const UUID_DCE: uuid_format = 2;
// pub const UUID_NTFS: uuid_format = 1;
// pub const UUID_DOS: uuid_format = 0;

/*
 * volume_id - reads filesystem label and uuid
 *
 * Copyright (C) 2004 Kay Sievers <kay.sievers@vrfy.org>
 *
 *	This library is free software; you can redistribute it and/or
 *	modify it under the terms of the GNU Lesser General Public
 *	License as published by the Free Software Foundation; either
 *	version 2.1 of the License, or (at your option) any later version.
 *
 *	This library is distributed in the hope that it will be useful,
 *	but WITHOUT ANY WARRANTY; without even the implied warranty of
 *	MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
 *	Lesser General Public License for more details.
 *
 *	You should have received a copy of the GNU Lesser General Public
 *	License along with this library; if not, write to the Free Software
 *	Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA 02111-1307 USA
 */
//config:config FEATURE_VOLUMEID_XFS
//config:	bool "xfs filesystem"
//config:	default y
//config:	depends on VOLUMEID
//kbuild:lib-$(CONFIG_FEATURE_VOLUMEID_XFS) += xfs.o
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct xfs_super_block {
  pub magic: [uint8_t; 4],
  pub blocksize: uint32_t,
  pub dblocks: uint64_t,
  pub rblocks: uint64_t,
  pub dummy1: [uint32_t; 2],
  pub uuid: [uint8_t; 16],
  pub dummy2: [uint32_t; 15],
  pub fname: [uint8_t; 12],
  pub dummy3: [uint32_t; 2],
  pub icount: uint64_t,
  pub ifree: uint64_t,
  pub fdblocks: uint64_t,
}
/*
 * volume_id - reads filesystem label and uuid
 *
 * Copyright (C) 2005 Kay Sievers <kay.sievers@vrfy.org>
 *
 *	This library is free software; you can redistribute it and/or
 *	modify it under the terms of the GNU Lesser General Public
 *	License as published by the Free Software Foundation; either
 *	version 2.1 of the License, or (at your option) any later version.
 *
 *	This library is distributed in the hope that it will be useful,
 *	but WITHOUT ANY WARRANTY; without even the implied warranty of
 *	MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
 *	Lesser General Public License for more details.
 *
 *	You should have received a copy of the GNU Lesser General Public
 *	License along with this library; if not, write to the Free Software
 *	Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA 02111-1307 USA
 */
/* #define dbg(...) bb_error_msg(__VA_ARGS__) */
/* volume_id.h */
//	int		fd_close:1;
//	uint8_t		label_raw[VOLUME_ID_LABEL_SIZE];
//	size_t		label_raw_len;
//	uint8_t		uuid_raw[VOLUME_ID_UUID_SIZE];
//	size_t		uuid_raw_len;
/* uuid is stored in ASCII (not binary) form here: */
//	char		type_version[VOLUME_ID_FORMAT_SIZE];
//	smallint	usage_id;
//	const char	*usage;
/*uint64_t off,*/
/* util.h */
/* size of superblock buffer, reiserfs block is at 64k */
/* size of seek buffer, FAT cluster is 32k max */
/* volume_id_set_uuid(id,buf,fmt) assumes size of uuid buf
 * by shifting: 4 << fmt, except for fmt == UUID_DCE_STRING.
 * The constants below should match sizes.
 */
/* 4 bytes */
/* 8 bytes */
/* 16 bytes */
/* 36 bytes (VOLUME_ID_UUID_SIZE) */
//void volume_id_set_usage(struct volume_id *id, enum volume_id_usage usage_id);
//void volume_id_set_usage_part(struct volume_id_partition *part, enum volume_id_usage usage_id);
//void volume_id_set_label_raw(struct volume_id *id, const uint8_t *buf, size_t count);
/* Probe routines */
/* RAID */
//int FAST_FUNC volume_id_probe_highpoint_37x_raid(struct volume_id *id /*,uint64_t off*/);
//int FAST_FUNC volume_id_probe_highpoint_45x_raid(struct volume_id *id /*,uint64_t off*/, uint64_t size);
//int FAST_FUNC volume_id_probe_intel_software_raid(struct volume_id *id /*,uint64_t off*/, uint64_t size);
/*,uint64_t off*/
//int FAST_FUNC volume_id_probe_lsi_mega_raid(struct volume_id *id /*,uint64_t off*/, uint64_t size);
//int FAST_FUNC volume_id_probe_nvidia_raid(struct volume_id *id /*,uint64_t off*/, uint64_t size);
//int FAST_FUNC volume_id_probe_promise_fasttrack_raid(struct volume_id *id /*,uint64_t off*/, uint64_t size);
//int FAST_FUNC volume_id_probe_silicon_medley_raid(struct volume_id *id /*,uint64_t off*/, uint64_t size);
//int FAST_FUNC volume_id_probe_via_raid(struct volume_id *id /*,uint64_t off*/, uint64_t size);
//int FAST_FUNC volume_id_probe_lvm1(struct volume_id *id /*,uint64_t off*/);
//int FAST_FUNC volume_id_probe_lvm2(struct volume_id *id /*,uint64_t off*/);
/* FS */
/*,uint64_t off*/
/*,uint64_t off*/
/*,uint64_t off*/
/*,uint64_t off*/
/*,uint64_t off*/
/*,uint64_t off*/
//int FAST_FUNC volume_id_probe_hpfs(struct volume_id *id /*,uint64_t off*/);
/*,uint64_t off*/
/*,uint64_t off*/
/*,uint64_t off*/
/*,uint64_t off*/
/*,uint64_t off*/
//int FAST_FUNC volume_id_probe_mac_partition_map(struct volume_id *id /*,uint64_t off*/);
/*, uint64_t off*/
//int FAST_FUNC volume_id_probe_msdos_part_table(struct volume_id *id /*,uint64_t off*/);
/*,uint64_t off*/
/*,uint64_t off*/
/*,uint64_t off*/
/*,uint64_t off*/
/*,uint64_t off*/
/*,uint64_t off*/
/*,uint64_t off*/
/*,uint64_t off*/
/*,uint64_t off*/
/*,uint64_t off*/
//int FAST_FUNC volume_id_probe_ufs(struct volume_id *id /*,uint64_t off*/);
#[no_mangle]
pub unsafe extern "C" fn volume_id_probe_xfs(mut id: *mut volume_id) -> libc::c_int
/*,uint64_t off*/ {
  let mut xs: *mut xfs_super_block = 0 as *mut xfs_super_block;
  xs = volume_id_get_buffer(id, 0i32 as uint64_t, 0x200i32 as size_t) as *mut xfs_super_block;
  if xs.is_null() {
    return -1i32;
  }
  if memcmp(
    (*xs).magic.as_mut_ptr() as *const libc::c_void,
    b"XFSB\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
    4i32 as libc::c_ulong,
  ) != 0i32
  {
    return -1i32;
  }
  //	volume_id_set_label_raw(id, xs->fname, 12);
  volume_id_set_label_string(id, (*xs).fname.as_mut_ptr(), 12i32 as size_t);
  volume_id_set_uuid(id, (*xs).uuid.as_mut_ptr(), UUID_DCE);
  //	volume_id_set_usage(id, VOLUME_ID_FILESYSTEM);
  (*id).type_0 = b"xfs\x00" as *const u8 as *const libc::c_char;
  return 0i32;
}
