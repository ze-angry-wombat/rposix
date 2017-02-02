use {int_t, ssize_t};
use time::{timespec};
use signal::{sigevent};

pub struct aiocb
{
    pub aio_fildes: ::int_t,
    pub aio_lio_opcode: ::int_t,
    pub aio_reqprio: ::int_t,
    pub aio_buf: *mut ::void_t,
    pub aio_nbytes: ::size_t,
    __next_prio: *mut aiocb,
    __abs_prio: ::int_t,
    __policy: ::int_t,
    __error_code: ::int_t,
    __return_value: ::ssize_t,
    pub aio_offset: ::sys::types::off_t,
    __pad: [::char_t; 0usize],
}

new!(aiocb);

pub const AIO_CANCELED:     ::uint_t = 0;
pub const AIO_NOTCANCELED:  ::uint_t = 1;
pub const AIO_ALLDONE:      ::uint_t = 2;
pub const LIO_READ:         ::uint_t = 0;
pub const LIO_WRITE:        ::uint_t = 1;
pub const LIO_NOP:          ::uint_t = 2;
pub const LIO_WAIT:         ::uint_t = 0;
pub const LIO_NOWAIT:       ::uint_t = 1;

pub fn aio_cancel(fd: int_t, aiocb: &mut aiocb) -> int_t
{
  extern
  {
    fn aio_cancel(fd: int_t, aiocbp: *mut aiocb) -> int_t;
  }
  unsafe
  {
    aio_cancel(fd, aiocb as *mut _)
  }
}

pub fn aio_error(aiocbp: &aiocb) -> int_t
{
  extern
  {
    fn aio_error(aiocbp: *const aiocb) -> int_t;
  }
  unsafe
  {
    aio_error(aiocbp as *const _)
  }
}

pub fn aio_fsync(op: int_t, aiocb: &mut aiocb) -> int_t
{
  extern
  {
    fn aio_fsync(op: int_t, aiocb: *mut aiocb) -> int_t;
  }
  unsafe
  {
    aio_fsync(op, aiocb as *mut _)
  }
}

pub fn aio_read(aiocb: &mut aiocb) -> int_t
{
  extern
  {
    fn aio_read(aiocbp: *mut aiocb) -> int_t;
  }
  unsafe
  {
    aio_read(aiocbp as *mut _)
  }
}

pub fn aio_return(aiocbp: &mut aiocb) -> ssize_t
{
  extern
  {
    fn aio_return(aiocbp: *mut aiocb) -> ssize_t;
  }
  unsafe
  {
    aio_return(aiocbp as *mut _)
  }
}

pub fn aio_suspend(list: &[&aiocb], timeout: &timespec) -> int_t
{
  extern
  {
    fn aio_suspend(list: *const *const aiocb, nent: int_t, timeout: *const timespec) -> int_t;
  }
  unsafe
  {
    aio_suspend(list.as_ptr() as *const _, list.len() as int_t, timeout as *const _)
  }
}

pub fn aio_write(aiocbp: &mut aiocb) -> int_t
{
  extern
  {
    fn aio_write(aiocbp: *mut aiocb) -> int_t;
  }
  unsafe
  {
    aio_write(aiocbp as *mut _)
  }
}

pub fn lio_listio(mode: int_t, list: &[&mut aiocb], sevp: &mut sigevent) -> int_t
{
  extern
  {
    fn lio_listio(mode: int_t, list: *const *mut aiocb, nent: int_t, sig: *mut sigevent) -> int_t;
  }
  unsafe
  {
    lio_listio(mode, list.as_ptr() as *const _, list.len() as int_t, sevp as *mut _)
  }
}
