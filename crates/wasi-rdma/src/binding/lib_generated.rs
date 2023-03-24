// This file is automatically generated, DO NOT EDIT
//
// To regenerate this file run the `crates/witx-bindgen` command

use core::fmt;
use core::mem::MaybeUninit;
#[repr(transparent)]
#[derive(Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct RdmaError(u16);
pub const RDMA_ERROR_SUCCESS: RdmaError = RdmaError(0);
pub const RDMA_ERROR_RUNTIME_ERROR: RdmaError = RdmaError(1);
pub const RDMA_ERROR_IO_ERROR: RdmaError = RdmaError(2);
impl RdmaError {
    pub const fn raw(&self) -> u16 {
        self.0
    }

    pub fn name(&self) -> &'static str {
        match self.0 {
            0 => "SUCCESS",
            1 => "RUNTIME_ERROR",
            2 => "IO_ERROR",
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
    pub fn message(&self) -> &'static str {
        match self.0 {
            0 => "",
            1 => "",
            2 => "",
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}
impl fmt::Debug for RdmaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RdmaError")
            .field("code", &self.0)
            .field("name", &self.name())
            .field("message", &self.message())
            .finish()
    }
}

pub type IbvMr = u32;
pub type IbvWc = u32;
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct RdmaAddrinfoStruct {
    pub flags: i32,
    pub port_space: i32,
    pub family: i32,
    pub qp_type: i32,
    pub src_len: u32,
    pub dst_len: u32,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct IbvQpCap {
    pub max_send_wr: u32,
    pub max_recv_wr: u32,
    pub max_send_sge: u32,
    pub max_recv_sge: u32,
    pub max_inline_data: u32,
}
pub type Rdma = u32;
pub type RdmaCq = u32;
pub type EpPd = u32;
pub unsafe fn rdma_init(
    node: &str,
    service: &str,
    hints: RdmaAddrinfoStruct,
    cap: IbvQpCap,
    is_server: u8,
) -> Result<Rdma, RdmaError> {
    let mut rp0 = MaybeUninit::<Rdma>::uninit();
    let ret = wasi_ephemeral_rdma::rdma_init(
        node.as_ptr() as i32,
        node.len() as i32,
        service.as_ptr() as i32,
        service.len() as i32,
        &hints as *const _ as i32,
        &cap as *const _ as i32,
        is_server as i32,
        rp0.as_mut_ptr() as i32,
    );
    match ret {
        0 => Ok(core::ptr::read(rp0.as_mut_ptr() as i32 as *const Rdma)),
        _ => Err(RdmaError(ret as u16)),
    }
}

pub unsafe fn rdma_connect(rdma: Rdma) -> Result<(), RdmaError> {
    let ret = wasi_ephemeral_rdma::rdma_connect(rdma as i32);
    match ret {
        0 => Ok(()),
        _ => Err(RdmaError(ret as u16)),
    }
}

pub unsafe fn rdma_disconnect(rdma: Rdma) -> Result<(), RdmaError> {
    let ret = wasi_ephemeral_rdma::rdma_disconnect(rdma as i32);
    match ret {
        0 => Ok(()),
        _ => Err(RdmaError(ret as u16)),
    }
}

pub unsafe fn rdma_get_send_comp(rdma: Rdma, wc: IbvWc) -> Result<IbvWc, RdmaError> {
    let mut rp0 = MaybeUninit::<IbvWc>::uninit();
    let ret =
        wasi_ephemeral_rdma::rdma_get_send_comp(rdma as i32, wc as i32, rp0.as_mut_ptr() as i32);
    match ret {
        0 => Ok(core::ptr::read(rp0.as_mut_ptr() as i32 as *const IbvWc)),
        _ => Err(RdmaError(ret as u16)),
    }
}

pub unsafe fn rdma_get_recv_comp(rdma: Rdma, wc: IbvWc) -> Result<IbvWc, RdmaError> {
    let mut rp0 = MaybeUninit::<IbvWc>::uninit();
    let ret =
        wasi_ephemeral_rdma::rdma_get_recv_comp(rdma as i32, wc as i32, rp0.as_mut_ptr() as i32);
    match ret {
        0 => Ok(core::ptr::read(rp0.as_mut_ptr() as i32 as *const IbvWc)),
        _ => Err(RdmaError(ret as u16)),
    }
}

pub unsafe fn rdma_reg_msgs(rdma: Rdma, addr: *mut u8, size: u32) -> Result<IbvMr, RdmaError> {
    let mut rp0 = MaybeUninit::<IbvMr>::uninit();
    let ret = wasi_ephemeral_rdma::rdma_reg_msgs(
        rdma as i32,
        addr as i32,
        size as i32,
        rp0.as_mut_ptr() as i32,
    );
    match ret {
        0 => Ok(core::ptr::read(rp0.as_mut_ptr() as i32 as *const IbvMr)),
        _ => Err(RdmaError(ret as u16)),
    }
}

pub unsafe fn rdma_dereg_mr(ibv_mr: IbvMr) {
    wasi_ephemeral_rdma::rdma_dereg_mr(ibv_mr as i32);
}

pub unsafe fn rdma_post_send(
    rdma: Rdma,
    addr: *mut u8,
    size: u32,
    send_mr: IbvMr,
    flags: u32,
) -> Result<(), RdmaError> {
    let ret = wasi_ephemeral_rdma::rdma_post_send(
        rdma as i32,
        addr as i32,
        size as i32,
        send_mr as i32,
        flags as i32,
    );
    match ret {
        0 => Ok(()),
        _ => Err(RdmaError(ret as u16)),
    }
}

pub unsafe fn rdma_post_recv(
    rdma: Rdma,
    addr: *mut u8,
    size: u32,
    recv_mr: IbvMr,
) -> Result<(), RdmaError> {
    let ret =
        wasi_ephemeral_rdma::rdma_post_recv(rdma as i32, addr as i32, size as i32, recv_mr as i32);
    match ret {
        0 => Ok(()),
        _ => Err(RdmaError(ret as u16)),
    }
}

pub unsafe fn ibv_query_qp(rdma: Rdma, ibv_qp_attrmask: u32) -> Result<(), RdmaError> {
    let ret = wasi_ephemeral_rdma::ibv_query_qp(rdma as i32, ibv_qp_attrmask as i32);
    match ret {
        0 => Ok(()),
        _ => Err(RdmaError(ret as u16)),
    }
}

pub unsafe fn print_hello_world() {
    wasi_ephemeral_rdma::print_hello_world();
}

pub mod wasi_ephemeral_rdma {
    #[link(wasm_import_module = "wasi_ephemeral_rdma")]
    extern "C" {
        pub fn rdma_init(
            arg0: i32,
            arg1: i32,
            arg2: i32,
            arg3: i32,
            arg4: i32,
            arg5: i32,
            arg6: i32,
            arg7: i32,
        ) -> i32;
        pub fn rdma_connect(arg0: i32) -> i32;
        pub fn rdma_disconnect(arg0: i32) -> i32;
        pub fn rdma_get_send_comp(arg0: i32, arg1: i32, arg2: i32) -> i32;
        pub fn rdma_get_recv_comp(arg0: i32, arg1: i32, arg2: i32) -> i32;
        pub fn rdma_reg_msgs(arg0: i32, arg1: i32, arg2: i32, arg3: i32) -> i32;
        pub fn rdma_dereg_mr(arg0: i32);
        pub fn rdma_post_send(arg0: i32, arg1: i32, arg2: i32, arg3: i32, arg4: i32) -> i32;
        pub fn rdma_post_recv(arg0: i32, arg1: i32, arg2: i32, arg3: i32) -> i32;
        pub fn ibv_query_qp(arg0: i32, arg1: i32) -> i32;
        pub fn print_hello_world();
    }
}
