//! Process management syscalls
use crate::task::{change_program_brk, exit_current_and_run_next, suspend_current_and_run_next,
    get_syscall_time, current_user_token, allocate_certain_memory, deallocate_certain_memory};
use crate::mm::translated_byte_buffer;
use crate::timer::get_time_us;
use crate::config::PAGE_SIZE;
use crate::mm::{VirtAddr, PageTable};
#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    if let Some(buffers) = translated_byte_buffer(current_user_token(), _ts  as *const u8, 16) {

        let us = get_time_us();
        let tv: TimeVal = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
        let src = unsafe {
            core::slice::from_raw_parts(&tv as *const TimeVal as *const u8, 16)
        };
        let mut offset = 0;
        for buffer in buffers {
            let n = buffer.len().min(16 - offset);
            buffer[..n].copy_from_slice(&src[offset..offset + n]);
            offset += n;
        }
        0
    }else {
        -1
    }
}

/// TODO: Finish sys_trace to pass testcases
/// HINT: You might reimplement it with virtual memory management.
pub fn sys_trace(_trace_request: usize, _id: usize, _data: usize) -> isize {
    trace!("kernel: sys_trace");
    match _trace_request {
        0 => {
            if _id > (1 << 38) {
                return -1;
            }
            let mut num: isize = -1;
            if let Some(buffers) = translated_byte_buffer(current_user_token(), _id as *const u8, 1) {
                for buffer in buffers {
                    num = buffer[0] as isize;
                }
                num as isize
            }else {
                -1
            }
        }
        1 => {
            if _id > (1 << 38) {
                return -1;
            }
            if let Some(mut buffers) = translated_byte_buffer(current_user_token(), _id as *const u8, 1) {
                let pte = PageTable::from_token(current_user_token())
                    .translate(VirtAddr::from(_id).floor());
                match pte { 
                    Some(p) if p.is_valid() && p.writable() => {
                        buffers[0][0] = _data as u8;
                        0
                    } 
                    _ => return -1 
                }
            }else {
                -1
            }
        }
        2 => {
            get_syscall_time(_id)
        }
        _ => {
            -1
        }
    }
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, _prot: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    if _start % PAGE_SIZE != 0 || _prot & !0x7 != 0 || _prot & 0x7 == 0 {
        -1
    }else {
        allocate_certain_memory(_start, _len, _prot)
    }
    
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    if _start % PAGE_SIZE != 0 {
        -1
    }else {
        deallocate_certain_memory(_start, _len)
    }
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
