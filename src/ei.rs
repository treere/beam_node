#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

mod internal {
    include!("./bindings.rs");
}

use std::ffi::CString;
use std::mem::MaybeUninit;

pub use internal::ERL_ATOM_EXT;
pub use internal::ERL_ERROR;
pub use internal::ERL_PID_EXT;
pub use internal::ERL_SMALL_TUPLE_EXT;
pub use internal::ERL_TICK;

pub use internal::ei_cnode;

pub use internal::erlang_msg;
pub use internal::erlang_pid;

pub struct Buff(pub internal::ei_x_buff);

impl Drop for Buff {
    fn drop(&mut self) {
        unsafe { internal::ei_x_free(&mut self.0) };
    }
}

pub fn ei_connect_init(this_node_name: &str, cookie: &str, creation: u32) -> Result<ei_cnode, ()> {
    let ec = MaybeUninit::<internal::ei_cnode>::uninit();
    let mut ec = unsafe { ec.assume_init() };

    let this_node_name = CString::new(this_node_name).unwrap();
    let cookie = CString::new(cookie).unwrap();

    let errno = unsafe {
        internal::ei_connect_init(&mut ec, this_node_name.as_ptr(), cookie.as_ptr(), creation)
    };

    if errno < 0 {
        Err(())
    } else {
        Ok(ec)
    }
}

pub fn ei_connect(node: &mut ei_cnode, name: &str) -> Result<i32, ()> {
    let name = CString::new(name).map_err(|_| ())?;

    let fd = unsafe { internal::ei_connect(node, name.as_ptr() as *mut i8) };

    if fd < 0 {
        Err(())
    } else {
        Ok(fd)
    }
}

pub fn ei_x_new() -> Buff {
    let buff = MaybeUninit::<internal::ei_x_buff>::uninit();
    let mut buff = unsafe { buff.assume_init() };

    unsafe { internal::ei_x_new(&mut buff) };

    Buff(buff)
}

pub fn ei_self(node: &mut ei_cnode) -> &erlang_pid {
    let res = unsafe { internal::ei_self(node) };

    unsafe { &*res }
}

pub fn ei_xreceive_msg(fd: ::std::os::raw::c_int) -> (Buff, erlang_msg, i32) {
    let msg = MaybeUninit::<erlang_msg>::uninit();
    let mut msg = unsafe { msg.assume_init() };

    let mut buff = ei_x_new();

    let resp = unsafe { internal::ei_xreceive_msg(fd, &mut msg, &mut buff.0) as i32 };

    (buff, msg, resp)
}

pub fn ei_xreceive_msg_tmo(fd: ::std::os::raw::c_int, tmo: u32) -> (Buff, erlang_msg, i32) {
    let msg = MaybeUninit::<erlang_msg>::uninit();
    let mut msg = unsafe { msg.assume_init() };

    let mut buff = ei_x_new();

    let resp = unsafe { internal::ei_xreceive_msg_tmo(fd, &mut msg, &mut buff.0, tmo) as i32 };

    (buff, msg, resp)
}

pub fn ei_reg_send(
    node: &mut ei_cnode,
    fd: ::std::os::raw::c_int,
    dst: &str,
    buff: &[u8],
) -> Result<(), ()> {
    let target = CString::new(dst).unwrap();

    if unsafe {
        internal::ei_reg_send(
            node,
            fd,
            target.as_ptr() as *mut i8,
            buff.as_ptr() as *mut i8,
            buff.len() as i32,
        )
    } < 0
    {
        Err(())
    } else {
        Ok(())
    }
}

pub fn ei_send(
    fd: ::std::os::raw::c_int,
    pid: &mut internal::erlang_pid,
    buff: &[u8],
) -> Result<(), ()> {
    if unsafe { internal::ei_send(fd, pid, buff.as_ptr() as *mut i8, buff.len() as i32) } < 0 {
        Err(())
    } else {
        Ok(())
    }
}
