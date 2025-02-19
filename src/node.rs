use erlang_term::{RawTerm, Term};

use crate::{
    ei,
    error::{EiError, EiResult},
};

#[derive(Debug)]
pub struct Pid {
    pub name: String,
    pub id: u32,
    pub serial: u32,
    pub creation: u32,
}

impl From<&Pid> for ei::erlang_pid {
    fn from(pid: &Pid) -> Self {
        let mut x = [0; 1021];
        pid.name
            .as_bytes()
            .iter()
            .enumerate()
            .for_each(|(index, b)| {
                x[index] = *b as i8;
            });

        ei::erlang_pid {
            num: pid.id,
            serial: pid.serial,
            creation: pid.creation as u32,
            node: x,
        }
    }
}

impl From<Pid> for Term {
    fn from(value: Pid) -> Self {
        RawTerm::NewPid {
            node: Box::new(RawTerm::Atom(value.name)),
            id: value.id,
            serial: value.serial,
            creation: value.creation,
        }
        .into()
    }
}

fn extract_pid(pid: &ei::erlang_pid) -> Pid {
    let pid_name = pid
        .node
        .iter()
        .take_while(|x| **x != 0)
        .map(|x| char::from_u32(*x as u32).unwrap());

    Pid {
        name: String::from_iter(pid_name),
        id: pid.num,
        serial: pid.serial,
        creation: pid.creation,
    }
}

pub struct Node(ei::ei_cnode);

impl Node {
    pub fn new(name: &str, cookie: &str) -> EiResult<Self> {
        ei::ei_connect_init(name, cookie, 0u32)
            .map_err(|_| EiError::Init)
            .map(|ec| Self(ec))
    }

    pub fn connect(mut self, server: &str) -> EiResult<Connection> {
        ei::ei_connect(&mut self.0, server)
            .map_err(|_| EiError::Connect)
            .map(|fd| Connection { node: self, fd })
    }

    pub fn my_pid(&mut self) -> Pid {
        let pid = ei::ei_self(&mut self.0);
        extract_pid(pid)
    }
}

pub struct Connection {
    node: Node,
    fd: ::std::os::raw::c_int,
}

impl Connection {
    pub fn reg_send<T: Into<RawTerm>>(&mut self, dst: &str, term: T) -> EiResult<()> {
        let term = term.into();
        ei::ei_reg_send(&mut self.node.0, self.fd, &dst, &term.to_bytes())
            .map_err(|_| EiError::Send)
    }

    pub fn send<T: Into<RawTerm>>(&mut self, pid: &Pid, term: T) -> EiResult<()> {
        let term = term.into();
        ei::ei_send(self.fd, &mut (pid.into()), &term.to_bytes()).map_err(|_| EiError::Send)
    }

    pub fn receive(&mut self) -> EiResult<RawTerm> {
        loop {
            let (buff, _msg, resp) = ei::ei_xreceive_msg(self.fd);

            if resp == (ei::ERL_TICK as i32) {
                continue;
            }
            if resp == (ei::ERL_ERROR as i32) {
                return Err(EiError::Receive);
            }

            let v = unsafe {
                std::slice::from_raw_parts(buff.0.buff as *const u8, buff.0.index as usize)
            };

            return RawTerm::from_bytes(v).map_err(|_| EiError::Decode);
        }
    }

    pub fn receive_tmo(&mut self, tmo: u32) -> EiResult<RawTerm> {
        loop {
            let (buff, _msg, resp) = ei::ei_xreceive_msg_tmo(self.fd, tmo);

            if resp == (ei::ERL_TICK as i32) {
                continue;
            }
            if resp == (ei::ERL_ERROR as i32) {
                return Err(EiError::Receive);
            }

            let v = unsafe {
                std::slice::from_raw_parts(buff.0.buff as *const u8, buff.0.index as usize)
            };

            return RawTerm::from_bytes(v).map_err(|_| EiError::Decode);
        }
    }
}
